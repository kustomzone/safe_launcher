// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

/// Intended for converting Launcher Errors into numeric codes for propagating some error information
/// across FFI boundaries and specially to C.
pub const LAUNCHER_ERROR_START_RANGE: i32 = ::safe_dns::errors::DNS_ERROR_START_RANGE - 500;

/// Launcher Errors
pub enum LauncherError {
    /// Error from safe_core. Boxed to hold a pointer instead of value so that this enum variant is
    /// not insanely bigger than others.
    CoreError(Box<::safe_core::errors::CoreError>),
    /// Errors from safe_nfs
    DnsError(Box<::safe_dns::errors::DnsError>),
    /// Errors from safe_nfs
    NfsError(Box<::safe_nfs::errors::NfsError>),
    /// Ipc Listener could not be bound to an endpoint
    IpcListenerCouldNotBeBound,
    /// The Ipc Listener has errored out. New apps will no longer be able to connect to Launcher
    IpcListenerAborted(::std::io::Error),
    /// The Ipc Stream could not be cloned
    IpcStreamCloneError(::std::io::Error),
    /// mpsc receiver has hung up
    ReceiverChannelDisconnected,
    /// IpcSession has been terminated due to either graceful shutdown or some error as indicated
    IpcSessionTerminated(Option<::std::io::Error>),
    /// Could not read the payload size from stream
    FailedReadingStreamPayloadSize,
    /// Could not write the payload size to stream
    FailedWritingStreamPayloadSize,
    /// Could not parse payload as a valid JSON
    JsonParseError(::rustc_serialize::json::ParserError),
    /// JSON non-conforming to the Launcher RFC
    SpecificParseError(String),
    /// Unable to find/traverse directory or file path
    PathNotFound,
    /// Supplied path was invalid
    InvalidPath,
    /// Permission denied - e.g. permission to access SAFEDrive etc.
    PermissionDenied,
    /// Error encoding into Json String
    JsonEncodeError(::rustc_serialize::json::EncoderError),
    /// Unexpected - Probably a Logic error
    Unexpected(String),
}

impl<'a> From<&'a str> for LauncherError {
    fn from(error: &'a str) -> LauncherError {
        LauncherError::Unexpected(error.to_string())
    }
}

impl From<::safe_core::errors::CoreError> for LauncherError {
    fn from(error: ::safe_core::errors::CoreError) -> LauncherError {
        LauncherError::CoreError(Box::new(error))
    }
}

impl From<::safe_nfs::errors::NfsError> for LauncherError {
    fn from(error: ::safe_nfs::errors::NfsError) -> LauncherError {
        LauncherError::NfsError(Box::new(error))
    }
}

impl From<::safe_dns::errors::DnsError> for LauncherError {
    fn from(error: ::safe_dns::errors::DnsError) -> LauncherError {
        LauncherError::DnsError(Box::new(error))
    }
}

impl From<::rustc_serialize::json::ParserError> for LauncherError {
    fn from(error: ::rustc_serialize::json::ParserError) -> LauncherError {
        LauncherError::JsonParseError(error)
    }
}

impl Into<i32> for LauncherError {
    fn into(self) -> i32 {
        match self {
            LauncherError::CoreError(error)                 => (*error).into(),
            LauncherError::NfsError(error)                  => (*error).into(),
            LauncherError::DnsError(error)                  => (*error).into(),
            LauncherError::IpcListenerCouldNotBeBound       => LAUNCHER_ERROR_START_RANGE - 1,
            LauncherError::IpcListenerAborted(_)            => LAUNCHER_ERROR_START_RANGE - 2,
            LauncherError::IpcStreamCloneError(_)           => LAUNCHER_ERROR_START_RANGE - 3,
            LauncherError::ReceiverChannelDisconnected      => LAUNCHER_ERROR_START_RANGE - 4,
            LauncherError::IpcSessionTerminated(_)          => LAUNCHER_ERROR_START_RANGE - 5,
            LauncherError::FailedReadingStreamPayloadSize   => LAUNCHER_ERROR_START_RANGE - 6,
            LauncherError::FailedWritingStreamPayloadSize   => LAUNCHER_ERROR_START_RANGE - 7,
            LauncherError::JsonParseError(_)                => LAUNCHER_ERROR_START_RANGE - 8,
            LauncherError::SpecificParseError(_)            => LAUNCHER_ERROR_START_RANGE - 9,
            LauncherError::PathNotFound                     => LAUNCHER_ERROR_START_RANGE - 10,
            LauncherError::InvalidPath                      => LAUNCHER_ERROR_START_RANGE - 11,
            LauncherError::PermissionDenied                 => LAUNCHER_ERROR_START_RANGE - 12,
            LauncherError::JsonEncodeError(_)               => LAUNCHER_ERROR_START_RANGE - 13,
            LauncherError::Unexpected(_)                    => LAUNCHER_ERROR_START_RANGE - 14,
        }
    }
}

impl ::std::fmt::Debug for LauncherError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            LauncherError::CoreError(ref error)             => write!(f, "LauncherError::CoreError -> {:?}", error),
            LauncherError::DnsError(ref error)              => write!(f, "LauncherError::DnsError -> {:?}", error),
            LauncherError::NfsError(ref error)              => write!(f, "LauncherError::NfsError -> {:?}", error),
            LauncherError::IpcListenerCouldNotBeBound       => write!(f, "LauncherError::IpcListenerCouldNotBeBound"),
            LauncherError::IpcListenerAborted(ref error)    => write!(f, "LauncherError::IpcListenerAborted -> {:?}", error),
            LauncherError::IpcStreamCloneError(ref error)   => write!(f, "LauncherError::IpcStreamCloneError -> {:?}", error),
            LauncherError::ReceiverChannelDisconnected      => write!(f, "LauncherError::ReceiverChannelDisconnected"),
            LauncherError::IpcSessionTerminated(ref error)  => write!(f, "LauncherError::IpcSessionTerminated -> {:?}", error),
            LauncherError::FailedReadingStreamPayloadSize   => write!(f, "LauncherError::FailedReadingStreamPayloadSize"),
            LauncherError::FailedWritingStreamPayloadSize   => write!(f, "LauncherError::FailedWritingStreamPayloadSize"),
            LauncherError::JsonParseError(ref error)        => write!(f, "LauncherError::JsonParseError -> {:?}", error),
            LauncherError::SpecificParseError(ref error)    => write!(f, "LauncherError::SpecificParseError -> {:?}", error),
            LauncherError::PathNotFound                     => write!(f, "LauncherError::PathNotFound"),
            LauncherError::InvalidPath                      => write!(f, "LauncherError::InvalidPath"),
            LauncherError::PermissionDenied                 => write!(f, "LauncherError::PermissionDenied"),
            LauncherError::JsonEncodeError(ref error)       => write!(f, "LauncherError::JsonEncodeError -> {:?}", error),
            LauncherError::Unexpected(ref error)            => write!(f, "LauncherError::Unexpected{{{:?}}}", error),
        }
    }
}
