// One of the critical aspects of this library is error handling. Because it is intended
// to communicate with an external device, any operation could discover a disconnection
// from the RN2903 serial link, so everything which does such communication will return
// a `Result<T, rn2903::Error>`.
#[macro_use] extern crate quick_error;
use std::io;

quick_error! {
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
type Result<T> = std::result::Result<T, Error>;

// It's first necessary to actually connect to the module. To this end, the library
// exports all the configuration information needed to configure a serial port to
// communicate correctly with an RN2903.

use serialport::prelude::*;
use std::io::prelude::*;
use core::convert::AsRef;
use std::ffi::OsStr;
use std::thread;
use core::time::Duration;

/// Returns a `serialport::SerialPortSettings` corresponding to the default settings of
/// an RNB2903. Use this to configure your serial port.
///
/// Information obtained from Microchip document 40001811 revision B. Timeout is by
/// default set to a very long time; this is sometimes modified on the `SerialPort` itself
/// during certain operations.
pub fn serial_config() -> SerialPortSettings {
    SerialPortSettings {
        baud_rate: 57600,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::new(1, 0)
    }
}

// Once connected to a serial port, the library needs to verify that it is actually
// connected to a RN2903 and not some other serial device. To this end, the `Rn2903`
// wrapper struct's `::new()` function checks the output of the `sys get ver` command,
// which is well-specified.

/// A handle to a serial link connected to a RN2903 module.
///
/// This library guarantees safety regardless of the state of the RN2903.
pub struct Rn2903 {
    port: Box<dyn SerialPort>,    
}

impl Rn2903 {
    /// Open a new connection to a module at the given path or port name, with the
    /// default settings.
    pub fn new_at<S: AsRef<OsStr>>(port_name: S) -> Result<Self> {
        let sp = serialport::open_with_settings(&port_name, &serial_config())?;
        Self::new(sp)
    }

    /// Open a new connection to a module over the given serial connection.
    pub fn new(mut port: Box<dyn SerialPort>) -> Result<Self> {
        let mut buf = [0; 35];
        port.write_all(b"sys get ver\x0D\x0A")?;
        port.flush()?;
        thread::sleep(Duration::from_millis(12));
        port.read(&mut buf)?;
        if &buf[0..6] != b"RN2903" {
            Err(Error::WrongDevice((&*String::from_utf8_lossy(&buf)).into()))
        } else {
            Ok(Self {
                port
            })
        }
    }

    /// Query the module for its firmware version information.
    ///
    /// Returns a `String` like `RN2903 1.0.3 Aug  8 2017 15:11:09`
    pub fn system_version(&mut self) -> Result<String> {
        let mut buf = [0; 35];
        self.port.write_all(b"sys get ver\x0D\x0A")?;
        self.port.flush()?;
        thread::sleep(Duration::from_millis(12));
        self.port.read(&mut buf)?;
        Ok((&*String::from_utf8_lossy(&buf).trim_end()).into())
    }
}

