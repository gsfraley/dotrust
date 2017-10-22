use com::IUnknown;
use winapi::minwindef::DWORD;
use winapi::winerror::HRESULT;

iid!(IID_ICLRRUNTIMEHOST = 0x90F1A06C, 0x7712, 0x4762, 0x86, 0xB5, 0x7A, 0x5E, 0xBA, 0x6B, 0xDB, 0x02);
com_interface! {
    interface ICLRRuntimeHost : IUnknown {
        iid: IID_ICLRRUNTIMEHOST,
        vtable: IClrRuntimeHostVtbl,
        fn start() -> HRESULT;
        fn stop() -> HRESULT;
        fn get_current_app_domain_id(app_domain_id: *mut DWORD) -> HRESULT;
    }
}