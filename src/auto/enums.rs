// This file was generated by gir (746446b) from gir-files (469db10)
// DO NOT EDIT

use ffi;
use glib_ffi;
use glib::error::ErrorDomain;
use glib::Type;
use glib::StaticType;
use glib::value::{Value, SetValue, FromValue, FromValueOptional};
use gobject_ffi;
use glib::translate::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum DataStreamByteOrder {
    BigEndian,
    LittleEndian,
    HostEndian,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for DataStreamByteOrder {
    type GlibType = ffi::GDataStreamByteOrder;

    fn to_glib(&self) -> ffi::GDataStreamByteOrder {
        match *self {
            DataStreamByteOrder::BigEndian => ffi::G_DATA_STREAM_BYTE_ORDER_BIG_ENDIAN,
            DataStreamByteOrder::LittleEndian => ffi::G_DATA_STREAM_BYTE_ORDER_LITTLE_ENDIAN,
            DataStreamByteOrder::HostEndian => ffi::G_DATA_STREAM_BYTE_ORDER_HOST_ENDIAN,
            DataStreamByteOrder::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GDataStreamByteOrder> for DataStreamByteOrder {
    fn from_glib(value: ffi::GDataStreamByteOrder) -> Self {
        match value {
            0 => DataStreamByteOrder::BigEndian,
            1 => DataStreamByteOrder::LittleEndian,
            2 => DataStreamByteOrder::HostEndian,
            value => DataStreamByteOrder::__Unknown(value),
        }
    }
}

impl StaticType for DataStreamByteOrder {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_data_stream_byte_order_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for DataStreamByteOrder {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for DataStreamByteOrder {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for DataStreamByteOrder {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum DataStreamNewlineType {
    Lf,
    Cr,
    CrLf,
    Any,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for DataStreamNewlineType {
    type GlibType = ffi::GDataStreamNewlineType;

    fn to_glib(&self) -> ffi::GDataStreamNewlineType {
        match *self {
            DataStreamNewlineType::Lf => ffi::G_DATA_STREAM_NEWLINE_TYPE_LF,
            DataStreamNewlineType::Cr => ffi::G_DATA_STREAM_NEWLINE_TYPE_CR,
            DataStreamNewlineType::CrLf => ffi::G_DATA_STREAM_NEWLINE_TYPE_CR_LF,
            DataStreamNewlineType::Any => ffi::G_DATA_STREAM_NEWLINE_TYPE_ANY,
            DataStreamNewlineType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GDataStreamNewlineType> for DataStreamNewlineType {
    fn from_glib(value: ffi::GDataStreamNewlineType) -> Self {
        match value {
            0 => DataStreamNewlineType::Lf,
            1 => DataStreamNewlineType::Cr,
            2 => DataStreamNewlineType::CrLf,
            3 => DataStreamNewlineType::Any,
            value => DataStreamNewlineType::__Unknown(value),
        }
    }
}

impl StaticType for DataStreamNewlineType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_data_stream_newline_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for DataStreamNewlineType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for DataStreamNewlineType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for DataStreamNewlineType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum FileType {
    Unknown,
    Regular,
    Directory,
    SymbolicLink,
    Special,
    Shortcut,
    Mountable,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for FileType {
    type GlibType = ffi::GFileType;

    fn to_glib(&self) -> ffi::GFileType {
        match *self {
            FileType::Unknown => ffi::G_FILE_TYPE_UNKNOWN,
            FileType::Regular => ffi::G_FILE_TYPE_REGULAR,
            FileType::Directory => ffi::G_FILE_TYPE_DIRECTORY,
            FileType::SymbolicLink => ffi::G_FILE_TYPE_SYMBOLIC_LINK,
            FileType::Special => ffi::G_FILE_TYPE_SPECIAL,
            FileType::Shortcut => ffi::G_FILE_TYPE_SHORTCUT,
            FileType::Mountable => ffi::G_FILE_TYPE_MOUNTABLE,
            FileType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GFileType> for FileType {
    fn from_glib(value: ffi::GFileType) -> Self {
        match value {
            0 => FileType::Unknown,
            1 => FileType::Regular,
            2 => FileType::Directory,
            3 => FileType::SymbolicLink,
            4 => FileType::Special,
            5 => FileType::Shortcut,
            6 => FileType::Mountable,
            value => FileType::__Unknown(value),
        }
    }
}

impl StaticType for FileType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_file_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for FileType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for FileType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for FileType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MountOperationResult {
    Handled,
    Aborted,
    Unhandled,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for MountOperationResult {
    type GlibType = ffi::GMountOperationResult;

    fn to_glib(&self) -> ffi::GMountOperationResult {
        match *self {
            MountOperationResult::Handled => ffi::G_MOUNT_OPERATION_HANDLED,
            MountOperationResult::Aborted => ffi::G_MOUNT_OPERATION_ABORTED,
            MountOperationResult::Unhandled => ffi::G_MOUNT_OPERATION_UNHANDLED,
            MountOperationResult::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GMountOperationResult> for MountOperationResult {
    fn from_glib(value: ffi::GMountOperationResult) -> Self {
        match value {
            0 => MountOperationResult::Handled,
            1 => MountOperationResult::Aborted,
            2 => MountOperationResult::Unhandled,
            value => MountOperationResult::__Unknown(value),
        }
    }
}

impl StaticType for MountOperationResult {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_mount_operation_result_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for MountOperationResult {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for MountOperationResult {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for MountOperationResult {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v2_42", feature = "dox"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum NotificationPriority {
    Normal,
    Low,
    High,
    Urgent,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v2_42", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for NotificationPriority {
    type GlibType = ffi::GNotificationPriority;

    fn to_glib(&self) -> ffi::GNotificationPriority {
        match *self {
            NotificationPriority::Normal => ffi::G_NOTIFICATION_PRIORITY_NORMAL,
            NotificationPriority::Low => ffi::G_NOTIFICATION_PRIORITY_LOW,
            NotificationPriority::High => ffi::G_NOTIFICATION_PRIORITY_HIGH,
            NotificationPriority::Urgent => ffi::G_NOTIFICATION_PRIORITY_URGENT,
            NotificationPriority::__Unknown(value) => value
        }
    }
}

#[cfg(any(feature = "v2_42", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<ffi::GNotificationPriority> for NotificationPriority {
    fn from_glib(value: ffi::GNotificationPriority) -> Self {
        match value {
            0 => NotificationPriority::Normal,
            1 => NotificationPriority::Low,
            2 => NotificationPriority::High,
            3 => NotificationPriority::Urgent,
            value => NotificationPriority::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "v2_42", feature = "dox"))]
impl StaticType for NotificationPriority {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_notification_priority_get_type()) }
    }
}

#[cfg(any(feature = "v2_42", feature = "dox"))]
impl<'a> FromValueOptional<'a> for NotificationPriority {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v2_42", feature = "dox"))]
impl<'a> FromValue<'a> for NotificationPriority {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v2_42", feature = "dox"))]
impl SetValue for NotificationPriority {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PasswordSave {
    Never,
    ForSession,
    Permanently,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for PasswordSave {
    type GlibType = ffi::GPasswordSave;

    fn to_glib(&self) -> ffi::GPasswordSave {
        match *self {
            PasswordSave::Never => ffi::G_PASSWORD_SAVE_NEVER,
            PasswordSave::ForSession => ffi::G_PASSWORD_SAVE_FOR_SESSION,
            PasswordSave::Permanently => ffi::G_PASSWORD_SAVE_PERMANENTLY,
            PasswordSave::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GPasswordSave> for PasswordSave {
    fn from_glib(value: ffi::GPasswordSave) -> Self {
        match value {
            0 => PasswordSave::Never,
            1 => PasswordSave::ForSession,
            2 => PasswordSave::Permanently,
            value => PasswordSave::__Unknown(value),
        }
    }
}

impl StaticType for PasswordSave {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_password_save_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PasswordSave {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PasswordSave {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for PasswordSave {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v2_34", feature = "dox"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ResolverRecordType {
    Srv,
    Mx,
    Txt,
    Soa,
    Ns,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v2_34", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for ResolverRecordType {
    type GlibType = ffi::GResolverRecordType;

    fn to_glib(&self) -> ffi::GResolverRecordType {
        match *self {
            ResolverRecordType::Srv => ffi::G_RESOLVER_RECORD_SRV,
            ResolverRecordType::Mx => ffi::G_RESOLVER_RECORD_MX,
            ResolverRecordType::Txt => ffi::G_RESOLVER_RECORD_TXT,
            ResolverRecordType::Soa => ffi::G_RESOLVER_RECORD_SOA,
            ResolverRecordType::Ns => ffi::G_RESOLVER_RECORD_NS,
            ResolverRecordType::__Unknown(value) => value
        }
    }
}

#[cfg(any(feature = "v2_34", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<ffi::GResolverRecordType> for ResolverRecordType {
    fn from_glib(value: ffi::GResolverRecordType) -> Self {
        match value {
            1 => ResolverRecordType::Srv,
            2 => ResolverRecordType::Mx,
            3 => ResolverRecordType::Txt,
            4 => ResolverRecordType::Soa,
            5 => ResolverRecordType::Ns,
            value => ResolverRecordType::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "v2_34", feature = "dox"))]
impl StaticType for ResolverRecordType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_resolver_record_type_get_type()) }
    }
}

#[cfg(any(feature = "v2_34", feature = "dox"))]
impl<'a> FromValueOptional<'a> for ResolverRecordType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v2_34", feature = "dox"))]
impl<'a> FromValue<'a> for ResolverRecordType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v2_34", feature = "dox"))]
impl SetValue for ResolverRecordType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ResourceError {
    NotFound,
    Internal,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for ResourceError {
    type GlibType = ffi::GResourceError;

    fn to_glib(&self) -> ffi::GResourceError {
        match *self {
            ResourceError::NotFound => ffi::G_RESOURCE_ERROR_NOT_FOUND,
            ResourceError::Internal => ffi::G_RESOURCE_ERROR_INTERNAL,
            ResourceError::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GResourceError> for ResourceError {
    fn from_glib(value: ffi::GResourceError) -> Self {
        match value {
            0 => ResourceError::NotFound,
            1 => ResourceError::Internal,
            value => ResourceError::__Unknown(value),
        }
    }
}

impl ErrorDomain for ResourceError {
    fn domain() -> glib_ffi::GQuark {
        unsafe { ffi::g_resource_error_quark() }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            0 => Some(ResourceError::NotFound),
            1 => Some(ResourceError::Internal),
            value => Some(ResourceError::__Unknown(value)),
        }
    }
}

impl StaticType for ResourceError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_resource_error_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ResourceError {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ResourceError {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for ResourceError {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SocketClientEvent {
    Resolving,
    Resolved,
    Connecting,
    Connected,
    ProxyNegotiating,
    ProxyNegotiated,
    TlsHandshaking,
    TlsHandshaked,
    Complete,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for SocketClientEvent {
    type GlibType = ffi::GSocketClientEvent;

    fn to_glib(&self) -> ffi::GSocketClientEvent {
        match *self {
            SocketClientEvent::Resolving => ffi::G_SOCKET_CLIENT_RESOLVING,
            SocketClientEvent::Resolved => ffi::G_SOCKET_CLIENT_RESOLVED,
            SocketClientEvent::Connecting => ffi::G_SOCKET_CLIENT_CONNECTING,
            SocketClientEvent::Connected => ffi::G_SOCKET_CLIENT_CONNECTED,
            SocketClientEvent::ProxyNegotiating => ffi::G_SOCKET_CLIENT_PROXY_NEGOTIATING,
            SocketClientEvent::ProxyNegotiated => ffi::G_SOCKET_CLIENT_PROXY_NEGOTIATED,
            SocketClientEvent::TlsHandshaking => ffi::G_SOCKET_CLIENT_TLS_HANDSHAKING,
            SocketClientEvent::TlsHandshaked => ffi::G_SOCKET_CLIENT_TLS_HANDSHAKED,
            SocketClientEvent::Complete => ffi::G_SOCKET_CLIENT_COMPLETE,
            SocketClientEvent::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GSocketClientEvent> for SocketClientEvent {
    fn from_glib(value: ffi::GSocketClientEvent) -> Self {
        match value {
            0 => SocketClientEvent::Resolving,
            1 => SocketClientEvent::Resolved,
            2 => SocketClientEvent::Connecting,
            3 => SocketClientEvent::Connected,
            4 => SocketClientEvent::ProxyNegotiating,
            5 => SocketClientEvent::ProxyNegotiated,
            6 => SocketClientEvent::TlsHandshaking,
            7 => SocketClientEvent::TlsHandshaked,
            8 => SocketClientEvent::Complete,
            value => SocketClientEvent::__Unknown(value),
        }
    }
}

impl StaticType for SocketClientEvent {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_socket_client_event_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SocketClientEvent {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SocketClientEvent {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for SocketClientEvent {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SocketFamily {
    Invalid,
    Unix,
    Ipv4,
    Ipv6,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for SocketFamily {
    type GlibType = ffi::GSocketFamily;

    fn to_glib(&self) -> ffi::GSocketFamily {
        match *self {
            SocketFamily::Invalid => ffi::G_SOCKET_FAMILY_INVALID,
            SocketFamily::Unix => ffi::G_SOCKET_FAMILY_UNIX,
            SocketFamily::Ipv4 => ffi::G_SOCKET_FAMILY_IPV4,
            SocketFamily::Ipv6 => ffi::G_SOCKET_FAMILY_IPV6,
            SocketFamily::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GSocketFamily> for SocketFamily {
    fn from_glib(value: ffi::GSocketFamily) -> Self {
        match value {
            0 => SocketFamily::Invalid,
            1 => SocketFamily::Unix,
            2 => SocketFamily::Ipv4,
            10 => SocketFamily::Ipv6,
            value => SocketFamily::__Unknown(value),
        }
    }
}

impl StaticType for SocketFamily {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_socket_family_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SocketFamily {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SocketFamily {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for SocketFamily {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v2_46", feature = "dox"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SocketListenerEvent {
    Binding,
    Bound,
    Listening,
    Listened,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v2_46", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for SocketListenerEvent {
    type GlibType = ffi::GSocketListenerEvent;

    fn to_glib(&self) -> ffi::GSocketListenerEvent {
        match *self {
            SocketListenerEvent::Binding => ffi::G_SOCKET_LISTENER_BINDING,
            SocketListenerEvent::Bound => ffi::G_SOCKET_LISTENER_BOUND,
            SocketListenerEvent::Listening => ffi::G_SOCKET_LISTENER_LISTENING,
            SocketListenerEvent::Listened => ffi::G_SOCKET_LISTENER_LISTENED,
            SocketListenerEvent::__Unknown(value) => value
        }
    }
}

#[cfg(any(feature = "v2_46", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<ffi::GSocketListenerEvent> for SocketListenerEvent {
    fn from_glib(value: ffi::GSocketListenerEvent) -> Self {
        match value {
            0 => SocketListenerEvent::Binding,
            1 => SocketListenerEvent::Bound,
            2 => SocketListenerEvent::Listening,
            3 => SocketListenerEvent::Listened,
            value => SocketListenerEvent::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "v2_46", feature = "dox"))]
impl StaticType for SocketListenerEvent {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_socket_listener_event_get_type()) }
    }
}

#[cfg(any(feature = "v2_46", feature = "dox"))]
impl<'a> FromValueOptional<'a> for SocketListenerEvent {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v2_46", feature = "dox"))]
impl<'a> FromValue<'a> for SocketListenerEvent {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v2_46", feature = "dox"))]
impl SetValue for SocketListenerEvent {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SocketProtocol {
    Unknown,
    Default,
    Tcp,
    Udp,
    Sctp,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for SocketProtocol {
    type GlibType = ffi::GSocketProtocol;

    fn to_glib(&self) -> ffi::GSocketProtocol {
        match *self {
            SocketProtocol::Unknown => ffi::G_SOCKET_PROTOCOL_UNKNOWN,
            SocketProtocol::Default => ffi::G_SOCKET_PROTOCOL_DEFAULT,
            SocketProtocol::Tcp => ffi::G_SOCKET_PROTOCOL_TCP,
            SocketProtocol::Udp => ffi::G_SOCKET_PROTOCOL_UDP,
            SocketProtocol::Sctp => ffi::G_SOCKET_PROTOCOL_SCTP,
            SocketProtocol::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GSocketProtocol> for SocketProtocol {
    fn from_glib(value: ffi::GSocketProtocol) -> Self {
        match value {
            -1 => SocketProtocol::Unknown,
            0 => SocketProtocol::Default,
            6 => SocketProtocol::Tcp,
            17 => SocketProtocol::Udp,
            132 => SocketProtocol::Sctp,
            value => SocketProtocol::__Unknown(value),
        }
    }
}

impl StaticType for SocketProtocol {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_socket_protocol_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SocketProtocol {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SocketProtocol {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for SocketProtocol {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SocketType {
    Invalid,
    Stream,
    Datagram,
    Seqpacket,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for SocketType {
    type GlibType = ffi::GSocketType;

    fn to_glib(&self) -> ffi::GSocketType {
        match *self {
            SocketType::Invalid => ffi::G_SOCKET_TYPE_INVALID,
            SocketType::Stream => ffi::G_SOCKET_TYPE_STREAM,
            SocketType::Datagram => ffi::G_SOCKET_TYPE_DATAGRAM,
            SocketType::Seqpacket => ffi::G_SOCKET_TYPE_SEQPACKET,
            SocketType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GSocketType> for SocketType {
    fn from_glib(value: ffi::GSocketType) -> Self {
        match value {
            0 => SocketType::Invalid,
            1 => SocketType::Stream,
            2 => SocketType::Datagram,
            3 => SocketType::Seqpacket,
            value => SocketType::__Unknown(value),
        }
    }
}

impl StaticType for SocketType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_socket_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SocketType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SocketType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for SocketType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

