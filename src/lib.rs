//! ## A Rusty interface for the RN2903 serial protocol
//!
//! The RN2903 is a LoRa and FSK transciever for the 915MHz ISM band, commonly used in USB
//! devices like the LoStik.
//!
//! This crate provides a safe, idiomatic interface using cross-platform native serial
//! functionality via `serialport`. This supports, for instance, a LoStik connected to a USB
//! TTY or virtual COM port, or a RN2903 connected via a TTL serial interface.
//!
//! See the [`Rn2903` struct](struct.Rn2903.html) for the bulk of the crate's functionality.

// One of the critical aspects of this library is error handling. Because it is intended
// to communicate with an external device, any operation could discover a disconnection
// from the RN2903 serial link, so everything which does such communication will return
// a `Result<T, rn2903::Error>`.
#[macro_use]
extern crate quick_error;
use std::io;

quick_error! {
    /// The primary error type used for fallible operations on the RN2903.
    #[derive(Debug)]
    pub enum Error {
        /// The connection to the RN2903 was impossible for some reason. Perhaps an
        /// invalid port was specified, or this program does not have permission to
        /// access the specified port.
        ConnectionFailed(err: serialport::Error) {
            cause(err)
            description(err.description())
            from()
        }
        /// The device to which the serial link is connected does not appear to be
        /// a RN2903, because it did not respond to `sys get ver` correctly.
        WrongDevice(version: String) {
            description("failed to verify connected module")
            display("Could not verify version string. Expected a RN2903 firmware revision, got '{}'",
                version)
        }
        /// The program has become disconnected from the RN2903 module due to an I/O
        /// error. It is possible the device was physically disconnected, or that the
        /// host operating system closed the serial port for some reason.
        Disconnected(err: io::Error) {
            cause(err)
            description(err.description())
            from()
        }
    }
}

/// Universal `Result` wrapper for the RN2903 interface.
pub type Result<T> = std::result::Result<T, Error>;

// It's first necessary to actually connect to the module. To this end, the library
// exports all the configuration information needed to configure a serial port to
// communicate correctly with an RN2903.

use core::convert::AsRef;
use core::time::Duration;
use serialport::prelude::*;
use std::ffi::OsStr;
use std::io::prelude::*;
use std::thread;

/// Returns the `SerialPortSettings` corresponding to the default settings of
/// an RNB2903.
///
/// Information obtained from Microchip document 40001811 revision B. Timeout is by
/// default set to a very long time; this is sometimes modified on the `SerialPort` itself
/// during certain operations.
///
/// # Examples
///
/// Opening a serial port with slightly modified settings. In this case, the baud rate
/// has been reduced. 
///
/// ```no_run
/// let settings = serialport::SerialPortSettings {
///     baud_rate: 9600,
///     ..rn2903::serial_config()
/// };
///
/// serialport::open_with_settings("/dev/ttyUSB0", &settings)
///     .expect("Could not open serial port. Error");
/// ```
pub fn serial_config() -> SerialPortSettings {
    SerialPortSettings {
        baud_rate: 57600,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::new(65535, 0),
    }
}

// Once connected to a serial port, the library needs to verify that it is actually
// connected to a RN2903 and not some other serial device. To this end, the `Rn2903`
// wrapper struct's `::new()` function checks the output of the `sys get ver` command,
// which is well-specified.

/// Turn the raw bytes into a String for display.
pub fn bytes_to_string(bytes: &[u8]) -> String {
    (&*String::from_utf8_lossy(bytes)).into()
}

/// A handle to a serial link connected to a RN2903 module.
///
/// This library guarantees safety regardless of the state of the RN2903. Refer to the
/// documentation for sections and individual associated functions for specifics.
///
/// # Examples
///
/// Basic functionality can be obtained just by using `::new_at()` and `::transact()`.
/// For instance, blinking the LoStik's LED:
///
/// ```no_run
/// # use rn2903::Rn2903;
/// # use std::time::Duration;
/// # use std::thread;
/// let mut txvr = Rn2903::new_at("/dev/ttyUSB0")
///     .expect("Could not open device. Error");
/// loop {
///     txvr.transact(b"radio set pindig GPIO10 0").unwrap();
///     thread::sleep(Duration::from_millis(1000));
///     txvr.transact(b"radio set pindig GPIO10 1").unwrap();
///     thread::sleep(Duration::from_millis(1000));
/// }
/// ```
pub struct Rn2903 {
    port: Box<dyn SerialPort>,
}

/// # Meta (type) Functions 
///
/// These functions deal with the type `Rn2903`, providing ways to create and manipulate
/// the structure itself. Aside from performing validation of the device on the other side
/// of the serial link, these functions do not communicate with the module.
///
/// ## Creating an `Rn2903`
/// There are several  ways to create a `Rn2903` wrapper for an RN2903 serial connection.
/// `::new_at()` is the recommended method, but `::new()` can be useful if the platform
/// does not support named serial ports, or some extra configuration is needed.
impl Rn2903 {
    /// Opens a new connection to a module at the given path or port name, with the
    /// default (and usually correct) settings from
    /// [`serial_config`](fn.serial_config.html).
    ///
    /// # Example
    /// 
    /// Connecting to a module accessible over the USB0 TTY.
    /// ```no_run
    /// # use rn2903::Rn2903;
    /// let txvr = Rn2903::new_at("/dev/ttyUSB0")
    ///     .expect("Could not open device. Error");
    /// ```
    pub fn new_at<S: AsRef<OsStr>>(port_name: S) -> Result<Self> {
        let sp = serialport::open_with_settings(&port_name, &serial_config())?;
        Self::new(sp)
    }

    /// Open a new connection to a module over the connection described by the given
    /// `SerialPort` trait object.
    pub fn new(port: Box<dyn SerialPort>) -> Result<Self> {
        let mut new = Self::new_unchecked(port);
        let version = new.system_version()?;
        if &version[0..6] != "RN2903" {
            Err(Error::WrongDevice(version))
        } else {
            Ok(new)
        }
    }

    /// Open a new connection to a module over the connection described by the given
    /// `SerialPort` trait object without performing a `sys get ver` check.
    ///
    /// The results of operations on a `Rn2903` struct that does _not_ represent an
    /// actual connection to an RN2903 module are completely unpredictable, and may
    /// result in lots of badness (though not memory unsafety).
    pub fn new_unchecked(port: Box<dyn SerialPort>) -> Self {
        Self { port }
    }
 
    /// Acquires temporary direct access to the captured `SerialPort` trait object.
    ///
    /// Use this access to, for example, reconfigure the connection on the fly,
    /// or set flags that will be used by devices this crate is unaware of.
    ///
    /// # Example
    ///
    /// Raising and then lowering the RTS signal, for example to signal a bus observer
    /// to switch on.
    /// ```no_run
    /// # use rn2903::Rn2903;
    /// # use std::thread;
    /// # use std::time::Duration;
    /// # let mut txvr = Rn2903::new_at("/dev/ttyUSB0")
    /// #    .expect("Could not open device. Error");
    /// txvr.port().write_request_to_send(true)
    ///     .expect("Could not set RTS. Error");
    /// thread::sleep(Duration::from_millis(25));
    /// txvr.port().write_request_to_send(false)
    ///     .expect("Could not set RTS. Error");
    /// ```
    pub fn port(&mut self) -> &mut dyn SerialPort {
        &mut *self.port
    }
}

/// # Low-level Communications 
impl Rn2903 {
    /// Writes the specified command to the module and returns a single line in response.
    ///
    /// This function adds the CRLF to the given command and returns the response without
    /// the CRLF.
    ///
    /// This is the preferred low-level communication method, since the RN2903 is supposed
    /// to respond with a single line to every command.
    pub fn transact(&mut self, command: &[u8]) -> Result<Vec<u8>> {
        self.send_line(command)?;
        self.read_line()
    }

    /// Writes the specified command to the module, adding a CRLF and flushing the buffer.
    ///
    /// Using [`::transact()`](#method.transact) is preferred.
    pub fn send_line(&mut self, line: &[u8]) -> Result<()> {
        use std::io::IoSlice;
        let bytes: Vec<u8> = line.iter().chain(b"\x0D\x0A".iter()).cloned().collect();
        let mut cursor = 0;
        while cursor < bytes.len() {
            cursor += self.port.write(&bytes[cursor..])?;
        } 
        self.port.flush()?;
        thread::sleep(Duration::from_millis(500));
        Ok(())
    }

    /// Reads bytes from the device until a CRLF is encountered, then returns the bytes
    /// read, not including the CRLF.
    ///
    /// Using [`::transact()`](#method.transact) is preferred.
    // This operation waits 12ms between each 32-byte read because the LoStick has
    // the hiccups.
    pub fn read_line(&mut self) -> Result<Vec<u8>> {
        let mut vec = Vec::with_capacity(32);
        loop {
            let mut buf = [0; 32];
            self.port.read(&mut buf)?;
            vec.extend_from_slice(&buf);

            // Check if crlf was added to the buffer.
            let mut found_lf = false;
            let mut found_crlf = false;
            for byte in vec.iter().rev() {
                if found_lf {
                    if *byte == b'\x0D' {
                        found_crlf = true;
                        break;
                    }
                } else {
                    found_lf = *byte == b'\x0A';
                }
            }
            if found_crlf {
                break;
            } else {
                thread::sleep(Duration::from_millis(12));
            }
        }

        // Remove zeroes and crlf
        while (b"\x00\x0D\x0A").contains(&vec[vec.len() - 1]) {
            vec.pop();
        }

        Ok(vec)
    }

}

/// # System API Functions
impl Rn2903 {
    /// Queries the module for its firmware version information.
    ///
    /// Returns a `String` like `RN2903 1.0.3 Aug  8 2017 15:11:09`
    pub fn system_version(&mut self) -> Result<String> {
        let bytes = self.transact(b"sys get ver")?;
        Ok(bytes_to_string(&bytes))
    } 
}
