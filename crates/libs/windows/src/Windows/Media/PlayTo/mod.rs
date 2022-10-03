#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentTimeChangeRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICurrentTimeChangeRequestedEventArgs {
    type Vtable = ICurrentTimeChangeRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ICurrentTimeChangeRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99711324_edc7_4bf5_91f6_3c8627db59e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentTimeChangeRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Time: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Time: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMuteChangeRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMuteChangeRequestedEventArgs {
    type Vtable = IMuteChangeRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMuteChangeRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4b4f5f6_af1f_4f1e_b437_7da32400e1d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMuteChangeRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Mute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPlayToConnection(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for IPlayToConnection {
    type Vtable = IPlayToConnection_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPlayToConnection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x112fbfc8_f235_4fde_8d41_9bf27c9e9a40);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToConnection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlayToConnectionState) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    State: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    StateChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveStateChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Transferred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Transferred: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveTransferred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveTransferred: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Error: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveError: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPlayToConnectionErrorEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for IPlayToConnectionErrorEventArgs {
    type Vtable = IPlayToConnectionErrorEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPlayToConnectionErrorEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf5eada6_88e6_445f_9d40_d9b9f8939896);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToConnectionErrorEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Code: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlayToConnectionError) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Code: usize,
    #[cfg(feature = "deprecated")]
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Message: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPlayToConnectionStateChangedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for IPlayToConnectionStateChangedEventArgs {
    type Vtable = IPlayToConnectionStateChangedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPlayToConnectionStateChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68c4b50f_0c20_4980_8602_58c62238d423);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToConnectionStateChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub PreviousState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlayToConnectionState) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PreviousState: usize,
    #[cfg(feature = "deprecated")]
    pub CurrentState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlayToConnectionState) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CurrentState: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPlayToConnectionTransferredEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for IPlayToConnectionTransferredEventArgs {
    type Vtable = IPlayToConnectionTransferredEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPlayToConnectionTransferredEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfae3193a_0683_47d9_8df0_18cbb48984d8);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToConnectionTransferredEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub PreviousSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PreviousSource: usize,
    #[cfg(feature = "deprecated")]
    pub CurrentSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CurrentSource: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPlayToManager(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for IPlayToManager {
    type Vtable = IPlayToManager_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPlayToManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf56a206e_1b77_42ef_8f0d_b949f8d9b260);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SourceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SourceRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSourceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSourceRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SourceSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SourceSelected: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSourceSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSourceSelected: usize,
    #[cfg(feature = "deprecated")]
    pub SetDefaultSourceSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetDefaultSourceSelection: usize,
    #[cfg(feature = "deprecated")]
    pub DefaultSourceSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DefaultSourceSelection: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPlayToManagerStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for IPlayToManagerStatics {
    type Vtable = IPlayToManagerStatics_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPlayToManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64e6a887_3982_4f3b_ba20_6155e435325b);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetForCurrentView: usize,
    #[cfg(feature = "deprecated")]
    pub ShowPlayToUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShowPlayToUI: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayToReceiver(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayToReceiver {
    type Vtable = IPlayToReceiver_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayToReceiver {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac15cf47_a162_4aa6_af1b_3aa35f3b9069);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToReceiver_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PlayRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlayRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlayRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlayRequested: usize,
    #[cfg(feature = "Foundation")]
    pub PauseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PauseRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePauseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePauseRequested: usize,
    #[cfg(feature = "Foundation")]
    pub SourceChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSourceChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSourceChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub PlaybackRateChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackRateChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlaybackRateChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlaybackRateChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub CurrentTimeChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CurrentTimeChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCurrentTimeChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCurrentTimeChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub MuteChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MuteChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMuteChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMuteChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub VolumeChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VolumeChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVolumeChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVolumeChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub TimeUpdateRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeUpdateRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTimeUpdateRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTimeUpdateRequested: usize,
    #[cfg(feature = "Foundation")]
    pub StopRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopRequested: usize,
    pub NotifyVolumeChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volume: f64, mute: bool) -> ::windows::core::HRESULT,
    pub NotifyRateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rate: f64) -> ::windows::core::HRESULT,
    pub NotifyLoadedMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NotifyTimeUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currenttime: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NotifyTimeUpdate: usize,
    #[cfg(feature = "Foundation")]
    pub NotifyDurationChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NotifyDurationChange: usize,
    pub NotifySeeking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NotifySeeked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NotifyPaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NotifyPlaying: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NotifyEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NotifyError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NotifyStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSupportsImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SupportsImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetSupportsAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SupportsAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetSupportsVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SupportsVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPlayToSource(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for IPlayToSource {
    type Vtable = IPlayToSource_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPlayToSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f138a08_fbb7_4b09_8356_aa5f4e335c31);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Connection: usize,
    #[cfg(feature = "deprecated")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Next: usize,
    #[cfg(feature = "deprecated")]
    pub SetNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetNext: usize,
    #[cfg(feature = "deprecated")]
    pub PlayNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PlayNext: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPlayToSourceDeferral(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for IPlayToSourceDeferral {
    type Vtable = IPlayToSourceDeferral_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPlayToSourceDeferral {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4100891d_278e_4f29_859b_a9e501053e7d);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceDeferral_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Complete: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPlayToSourceRequest(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for IPlayToSourceRequest {
    type Vtable = IPlayToSourceRequest_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPlayToSourceRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8584665_64f4_44a0_ac0d_468d2b8fda83);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Deadline: usize,
    #[cfg(feature = "deprecated")]
    pub DisplayErrorString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DisplayErrorString: usize,
    #[cfg(feature = "deprecated")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetDeferral: usize,
    #[cfg(feature = "deprecated")]
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetSource: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPlayToSourceRequestedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for IPlayToSourceRequestedEventArgs {
    type Vtable = IPlayToSourceRequestedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPlayToSourceRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5cdc330_29df_4ec6_9da9_9fbdfcfc1b3e);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub SourceRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SourceRequest: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPlayToSourceSelectedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for IPlayToSourceSelectedEventArgs {
    type Vtable = IPlayToSourceSelectedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPlayToSourceSelectedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c9d8511_5202_4dcb_8c67_abda12bb3c12);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceSelectedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FriendlyName: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub Icon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    Icon: usize,
    #[cfg(feature = "deprecated")]
    pub SupportsImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SupportsImage: usize,
    #[cfg(feature = "deprecated")]
    pub SupportsAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SupportsAudio: usize,
    #[cfg(feature = "deprecated")]
    pub SupportsVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SupportsVideo: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPlayToSourceWithPreferredSourceUri(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for IPlayToSourceWithPreferredSourceUri {
    type Vtable = IPlayToSourceWithPreferredSourceUri_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPlayToSourceWithPreferredSourceUri {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaab253eb_3301_4dc4_afba_b2f2ed9635a0);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceWithPreferredSourceUri_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PreferredSourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PreferredSourceUri: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetPreferredSourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetPreferredSourceUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaybackRateChangeRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlaybackRateChangeRequestedEventArgs {
    type Vtable = IPlaybackRateChangeRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlaybackRateChangeRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f5661ae_2c88_4cca_8540_d586095d13a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackRateChangeRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Rate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISourceChangeRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISourceChangeRequestedEventArgs {
    type Vtable = ISourceChangeRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ISourceChangeRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb3f3a96_7aa6_4a8b_86e7_54f6c6d34f64);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISourceChangeRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Stream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Stream: usize,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Album: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Genre: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Date: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Foundation")]
    pub Rating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Rating: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVolumeChangeRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IVolumeChangeRequestedEventArgs {
    type Vtable = IVolumeChangeRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IVolumeChangeRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f026d5c_cf75_4c2b_913e_6d7c6c329179);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVolumeChangeRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Volume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_PlayTo\"`*"]
#[repr(transparent)]
pub struct CurrentTimeChangeRequestedEventArgs(::windows::core::IUnknown);
impl CurrentTimeChangeRequestedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Time(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Time)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for CurrentTimeChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CurrentTimeChangeRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CurrentTimeChangeRequestedEventArgs {}
impl ::core::fmt::Debug for CurrentTimeChangeRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CurrentTimeChangeRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CurrentTimeChangeRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.CurrentTimeChangeRequestedEventArgs;{99711324-edc7-4bf5-91f6-3c8627db59e5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CurrentTimeChangeRequestedEventArgs {
    type Vtable = ICurrentTimeChangeRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for CurrentTimeChangeRequestedEventArgs {
    const IID: ::windows::core::GUID = <ICurrentTimeChangeRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CurrentTimeChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.CurrentTimeChangeRequestedEventArgs";
}
::windows::core::interface_hierarchy!(CurrentTimeChangeRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Media_PlayTo\"`*"]
#[repr(transparent)]
pub struct MuteChangeRequestedEventArgs(::windows::core::IUnknown);
impl MuteChangeRequestedEventArgs {
    pub fn Mute(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Mute)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for MuteChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MuteChangeRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MuteChangeRequestedEventArgs {}
impl ::core::fmt::Debug for MuteChangeRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MuteChangeRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MuteChangeRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.MuteChangeRequestedEventArgs;{e4b4f5f6-af1f-4f1e-b437-7da32400e1d4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MuteChangeRequestedEventArgs {
    type Vtable = IMuteChangeRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MuteChangeRequestedEventArgs {
    const IID: ::windows::core::GUID = <IMuteChangeRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MuteChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.MuteChangeRequestedEventArgs";
}
::windows::core::interface_hierarchy!(MuteChangeRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Media_PlayTo\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PlayToConnection(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToConnection {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn State(&self) -> ::windows::core::Result<PlayToConnectionState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).State)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayToConnectionState>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn StateChanged(&self, handler: &super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionStateChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StateChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStateChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Transferred(&self, handler: &super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionTransferredEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Transferred)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveTransferred(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveTransferred)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Error(&self, handler: &super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionErrorEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Error)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveError(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveError)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToConnection {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToConnection").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PlayToConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToConnection;{112fbfc8-f235-4fde-8d41-9bf27c9e9a40})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for PlayToConnection {
    type Vtable = IPlayToConnection_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PlayToConnection {
    const IID: ::windows::core::GUID = <IPlayToConnection as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PlayToConnection {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToConnection";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(PlayToConnection, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToConnection {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToConnection {}
#[doc = "*Required features: `\"Media_PlayTo\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PlayToConnectionErrorEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToConnectionErrorEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Code(&self) -> ::windows::core::Result<PlayToConnectionError> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Code)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayToConnectionError>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Message)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToConnectionErrorEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToConnectionErrorEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToConnectionErrorEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToConnectionErrorEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToConnectionErrorEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PlayToConnectionErrorEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToConnectionErrorEventArgs;{bf5eada6-88e6-445f-9d40-d9b9f8939896})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for PlayToConnectionErrorEventArgs {
    type Vtable = IPlayToConnectionErrorEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PlayToConnectionErrorEventArgs {
    const IID: ::windows::core::GUID = <IPlayToConnectionErrorEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PlayToConnectionErrorEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToConnectionErrorEventArgs";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(PlayToConnectionErrorEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToConnectionErrorEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToConnectionErrorEventArgs {}
#[doc = "*Required features: `\"Media_PlayTo\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PlayToConnectionStateChangedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToConnectionStateChangedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PreviousState(&self) -> ::windows::core::Result<PlayToConnectionState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviousState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayToConnectionState>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CurrentState(&self) -> ::windows::core::Result<PlayToConnectionState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CurrentState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayToConnectionState>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToConnectionStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToConnectionStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToConnectionStateChangedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToConnectionStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToConnectionStateChangedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PlayToConnectionStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToConnectionStateChangedEventArgs;{68c4b50f-0c20-4980-8602-58c62238d423})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for PlayToConnectionStateChangedEventArgs {
    type Vtable = IPlayToConnectionStateChangedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PlayToConnectionStateChangedEventArgs {
    const IID: ::windows::core::GUID = <IPlayToConnectionStateChangedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PlayToConnectionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToConnectionStateChangedEventArgs";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(PlayToConnectionStateChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToConnectionStateChangedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToConnectionStateChangedEventArgs {}
#[doc = "*Required features: `\"Media_PlayTo\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PlayToConnectionTransferredEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToConnectionTransferredEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PreviousSource(&self) -> ::windows::core::Result<PlayToSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviousSource)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayToSource>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CurrentSource(&self) -> ::windows::core::Result<PlayToSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CurrentSource)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayToSource>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToConnectionTransferredEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToConnectionTransferredEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToConnectionTransferredEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToConnectionTransferredEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToConnectionTransferredEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PlayToConnectionTransferredEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToConnectionTransferredEventArgs;{fae3193a-0683-47d9-8df0-18cbb48984d8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for PlayToConnectionTransferredEventArgs {
    type Vtable = IPlayToConnectionTransferredEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PlayToConnectionTransferredEventArgs {
    const IID: ::windows::core::GUID = <IPlayToConnectionTransferredEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PlayToConnectionTransferredEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToConnectionTransferredEventArgs";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(PlayToConnectionTransferredEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToConnectionTransferredEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToConnectionTransferredEventArgs {}
#[doc = "*Required features: `\"Media_PlayTo\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PlayToManager(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToManager {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SourceRequested(&self, handler: &super::super::Foundation::TypedEventHandler<PlayToManager, PlayToSourceRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SourceRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveSourceRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSourceRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SourceSelected(&self, handler: &super::super::Foundation::TypedEventHandler<PlayToManager, PlayToSourceSelectedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SourceSelected)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveSourceSelected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSourceSelected)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetDefaultSourceSelection(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDefaultSourceSelection)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn DefaultSourceSelection(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DefaultSourceSelection)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn GetForCurrentView() -> ::windows::core::Result<PlayToManager> {
        Self::IPlayToManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForCurrentView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayToManager>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ShowPlayToUI() -> ::windows::core::Result<()> {
        Self::IPlayToManagerStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).ShowPlayToUI)(::windows::core::Vtable::as_raw(this)).ok() })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IPlayToManagerStatics<R, F: FnOnce(&IPlayToManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlayToManager, IPlayToManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToManager {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToManager").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PlayToManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToManager;{f56a206e-1b77-42ef-8f0d-b949f8d9b260})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for PlayToManager {
    type Vtable = IPlayToManager_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PlayToManager {
    const IID: ::windows::core::GUID = <IPlayToManager as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PlayToManager {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToManager";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(PlayToManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToManager {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToManager {}
#[doc = "*Required features: `\"Media_PlayTo\"`*"]
#[repr(transparent)]
pub struct PlayToReceiver(::windows::core::IUnknown);
impl PlayToReceiver {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlayToReceiver, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PlayRequested(&self, handler: &super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PlayRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePlayRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePlayRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PauseRequested(&self, handler: &super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PauseRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePauseRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePauseRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SourceChangeRequested(&self, handler: &super::super::Foundation::TypedEventHandler<PlayToReceiver, SourceChangeRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SourceChangeRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSourceChangeRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSourceChangeRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PlaybackRateChangeRequested(&self, handler: &super::super::Foundation::TypedEventHandler<PlayToReceiver, PlaybackRateChangeRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PlaybackRateChangeRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePlaybackRateChangeRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePlaybackRateChangeRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CurrentTimeChangeRequested(&self, handler: &super::super::Foundation::TypedEventHandler<PlayToReceiver, CurrentTimeChangeRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CurrentTimeChangeRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCurrentTimeChangeRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveCurrentTimeChangeRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MuteChangeRequested(&self, handler: &super::super::Foundation::TypedEventHandler<PlayToReceiver, MuteChangeRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MuteChangeRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMuteChangeRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveMuteChangeRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VolumeChangeRequested(&self, handler: &super::super::Foundation::TypedEventHandler<PlayToReceiver, VolumeChangeRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VolumeChangeRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVolumeChangeRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveVolumeChangeRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeUpdateRequested(&self, handler: &super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TimeUpdateRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTimeUpdateRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveTimeUpdateRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StopRequested(&self, handler: &super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StopRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStopRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn NotifyVolumeChange(&self, volume: f64, mute: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NotifyVolumeChange)(::windows::core::Vtable::as_raw(this), volume, mute).ok() }
    }
    pub fn NotifyRateChange(&self, rate: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NotifyRateChange)(::windows::core::Vtable::as_raw(this), rate).ok() }
    }
    pub fn NotifyLoadedMetadata(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NotifyLoadedMetadata)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NotifyTimeUpdate(&self, currenttime: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NotifyTimeUpdate)(::windows::core::Vtable::as_raw(this), currenttime).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NotifyDurationChange(&self, duration: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NotifyDurationChange)(::windows::core::Vtable::as_raw(this), duration).ok() }
    }
    pub fn NotifySeeking(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NotifySeeking)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn NotifySeeked(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NotifySeeked)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn NotifyPaused(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NotifyPaused)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn NotifyPlaying(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NotifyPlaying)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn NotifyEnded(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NotifyEnded)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn NotifyError(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NotifyError)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn NotifyStopped(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NotifyStopped)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FriendlyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetFriendlyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetFriendlyName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SetSupportsImage(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSupportsImage)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SupportsImage(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportsImage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetSupportsAudio(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSupportsAudio)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SupportsAudio(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportsAudio)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetSupportsVideo(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSupportsVideo)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SupportsVideo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportsVideo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StopAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StopAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for PlayToReceiver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayToReceiver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayToReceiver {}
impl ::core::fmt::Debug for PlayToReceiver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToReceiver").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlayToReceiver {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToReceiver;{ac15cf47-a162-4aa6-af1b-3aa35f3b9069})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PlayToReceiver {
    type Vtable = IPlayToReceiver_Vtbl;
}
unsafe impl ::windows::core::Interface for PlayToReceiver {
    const IID: ::windows::core::GUID = <IPlayToReceiver as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlayToReceiver {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToReceiver";
}
::windows::core::interface_hierarchy!(PlayToReceiver, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Media_PlayTo\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PlayToSource(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToSource {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Connection(&self) -> ::windows::core::Result<PlayToConnection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Connection)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayToConnection>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Next(&self) -> ::windows::core::Result<PlayToSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Next)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayToSource>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetNext(&self, value: &PlayToSource) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetNext)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PlayNext(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).PlayNext)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn PreferredSourceUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IPlayToSourceWithPreferredSourceUri>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreferredSourceUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetPreferredSourceUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayToSourceWithPreferredSourceUri>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPreferredSourceUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToSource {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToSource").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PlayToSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToSource;{7f138a08-fbb7-4b09-8356-aa5f4e335c31})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for PlayToSource {
    type Vtable = IPlayToSource_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PlayToSource {
    const IID: ::windows::core::GUID = <IPlayToSource as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PlayToSource {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSource";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(PlayToSource, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToSource {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToSource {}
#[doc = "*Required features: `\"Media_PlayTo\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PlayToSourceDeferral(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToSourceDeferral {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Complete)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToSourceDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToSourceDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToSourceDeferral {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToSourceDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToSourceDeferral").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PlayToSourceDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToSourceDeferral;{4100891d-278e-4f29-859b-a9e501053e7d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for PlayToSourceDeferral {
    type Vtable = IPlayToSourceDeferral_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PlayToSourceDeferral {
    const IID: ::windows::core::GUID = <IPlayToSourceDeferral as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PlayToSourceDeferral {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSourceDeferral";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(PlayToSourceDeferral, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToSourceDeferral {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToSourceDeferral {}
#[doc = "*Required features: `\"Media_PlayTo\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PlayToSourceRequest(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToSourceRequest {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Deadline)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn DisplayErrorString(&self, errorstring: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).DisplayErrorString)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(errorstring)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<PlayToSourceDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayToSourceDeferral>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetSource(&self, value: &PlayToSource) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSource)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToSourceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToSourceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToSourceRequest {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToSourceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToSourceRequest").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PlayToSourceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToSourceRequest;{f8584665-64f4-44a0-ac0d-468d2b8fda83})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for PlayToSourceRequest {
    type Vtable = IPlayToSourceRequest_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PlayToSourceRequest {
    const IID: ::windows::core::GUID = <IPlayToSourceRequest as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PlayToSourceRequest {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSourceRequest";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(PlayToSourceRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToSourceRequest {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToSourceRequest {}
#[doc = "*Required features: `\"Media_PlayTo\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PlayToSourceRequestedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToSourceRequestedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SourceRequest(&self) -> ::windows::core::Result<PlayToSourceRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SourceRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayToSourceRequest>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToSourceRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToSourceRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToSourceRequestedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToSourceRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToSourceRequestedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PlayToSourceRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToSourceRequestedEventArgs;{c5cdc330-29df-4ec6-9da9-9fbdfcfc1b3e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for PlayToSourceRequestedEventArgs {
    type Vtable = IPlayToSourceRequestedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PlayToSourceRequestedEventArgs {
    const IID: ::windows::core::GUID = <IPlayToSourceRequestedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PlayToSourceRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSourceRequestedEventArgs";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(PlayToSourceRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToSourceRequestedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToSourceRequestedEventArgs {}
#[doc = "*Required features: `\"Media_PlayTo\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PlayToSourceSelectedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToSourceSelectedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FriendlyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn Icon(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Icon)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IRandomAccessStreamWithContentType>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SupportsImage(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportsImage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SupportsAudio(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportsAudio)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SupportsVideo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportsVideo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToSourceSelectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToSourceSelectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToSourceSelectedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToSourceSelectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToSourceSelectedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PlayToSourceSelectedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToSourceSelectedEventArgs;{0c9d8511-5202-4dcb-8c67-abda12bb3c12})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for PlayToSourceSelectedEventArgs {
    type Vtable = IPlayToSourceSelectedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PlayToSourceSelectedEventArgs {
    const IID: ::windows::core::GUID = <IPlayToSourceSelectedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PlayToSourceSelectedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSourceSelectedEventArgs";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(PlayToSourceSelectedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToSourceSelectedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToSourceSelectedEventArgs {}
#[doc = "*Required features: `\"Media_PlayTo\"`*"]
#[repr(transparent)]
pub struct PlaybackRateChangeRequestedEventArgs(::windows::core::IUnknown);
impl PlaybackRateChangeRequestedEventArgs {
    pub fn Rate(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Rate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for PlaybackRateChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlaybackRateChangeRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlaybackRateChangeRequestedEventArgs {}
impl ::core::fmt::Debug for PlaybackRateChangeRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackRateChangeRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlaybackRateChangeRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlaybackRateChangeRequestedEventArgs;{0f5661ae-2c88-4cca-8540-d586095d13a5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PlaybackRateChangeRequestedEventArgs {
    type Vtable = IPlaybackRateChangeRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for PlaybackRateChangeRequestedEventArgs {
    const IID: ::windows::core::GUID = <IPlaybackRateChangeRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlaybackRateChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlaybackRateChangeRequestedEventArgs";
}
::windows::core::interface_hierarchy!(PlaybackRateChangeRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Media_PlayTo\"`*"]
#[repr(transparent)]
pub struct SourceChangeRequestedEventArgs(::windows::core::IUnknown);
impl SourceChangeRequestedEventArgs {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Stream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Stream)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IRandomAccessStreamWithContentType>(result__)
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Author(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Author)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Album(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Album)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Genre(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Genre)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Description)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Date(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Date)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Thumbnail)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Rating(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Rating)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
}
impl ::core::clone::Clone for SourceChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SourceChangeRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SourceChangeRequestedEventArgs {}
impl ::core::fmt::Debug for SourceChangeRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SourceChangeRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SourceChangeRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.SourceChangeRequestedEventArgs;{fb3f3a96-7aa6-4a8b-86e7-54f6c6d34f64})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SourceChangeRequestedEventArgs {
    type Vtable = ISourceChangeRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for SourceChangeRequestedEventArgs {
    const IID: ::windows::core::GUID = <ISourceChangeRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SourceChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.SourceChangeRequestedEventArgs";
}
::windows::core::interface_hierarchy!(SourceChangeRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Media_PlayTo\"`*"]
#[repr(transparent)]
pub struct VolumeChangeRequestedEventArgs(::windows::core::IUnknown);
impl VolumeChangeRequestedEventArgs {
    pub fn Volume(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Volume)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for VolumeChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VolumeChangeRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VolumeChangeRequestedEventArgs {}
impl ::core::fmt::Debug for VolumeChangeRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VolumeChangeRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VolumeChangeRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.VolumeChangeRequestedEventArgs;{6f026d5c-cf75-4c2b-913e-6d7c6c329179})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for VolumeChangeRequestedEventArgs {
    type Vtable = IVolumeChangeRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for VolumeChangeRequestedEventArgs {
    const IID: ::windows::core::GUID = <IVolumeChangeRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VolumeChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.VolumeChangeRequestedEventArgs";
}
::windows::core::interface_hierarchy!(VolumeChangeRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Media_PlayTo\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PlayToConnectionError(pub i32);
#[cfg(feature = "deprecated")]
impl PlayToConnectionError {
    pub const None: Self = Self(0i32);
    pub const DeviceNotResponding: Self = Self(1i32);
    pub const DeviceError: Self = Self(2i32);
    pub const DeviceLocked: Self = Self(3i32);
    pub const ProtectedPlaybackFailed: Self = Self(4i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for PlayToConnectionError {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToConnectionError {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for PlayToConnectionError {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for PlayToConnectionError {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToConnectionError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToConnectionError").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PlayToConnectionError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.PlayTo.PlayToConnectionError;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_PlayTo\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PlayToConnectionState(pub i32);
#[cfg(feature = "deprecated")]
impl PlayToConnectionState {
    pub const Disconnected: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
    pub const Rendering: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for PlayToConnectionState {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToConnectionState {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for PlayToConnectionState {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for PlayToConnectionState {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToConnectionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToConnectionState").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PlayToConnectionState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.PlayTo.PlayToConnectionState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
