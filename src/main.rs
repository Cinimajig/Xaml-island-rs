mod desktop_target;
mod ro_runtime;

use bindings::Windows::Win32::System::SystemServices::*;
use bindings::Windows::Win32::UI::MenusAndResources::HMENU;
use bindings::Windows::Win32::UI::WindowsAndMessaging::*;
use bindings::Windows::Win32::Graphics::Gdi::{UpdateWindow, HBRUSH};
use bindings::Windows::Win32::UI::DisplayDevices::RECT;
use bindings::Windows::UI::Xaml::{Hosting::*, *};
use desktop_target::*;
use std::ptr::null_mut;
use windows::Interface;

const WND_CLASS_NAME: &str = env!("CARGO_PKG_NAME");
static mut CHILD_HWND: HWND = HWND::NULL;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", &err);
    }
}

fn run() -> windows::Result<()> {
    let h_instance = unsafe { GetModuleHandleW(PWSTR::NULL) };

    let mut class_name: Vec<u16> = WND_CLASS_NAME
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    let h_wnd = create_window(h_instance, class_name.as_mut_ptr())?;

    // The call to winrt::init_apartment initializes COM; by default, in a multithreaded apartment.
    let _ro = ro_runtime::RoInit::multi_threaded()?;

    // Initialize the XAML framework's core window for the current thread.
    let _win_xaml_manager = WindowsXamlManager::InitializeForCurrentThread()?;

    // This DesktopWindowXamlSource is the object that enables a non-UWP desktop application
    // to host WinRT XAML controls in any UI element that is associated with a window handle (HWND).
    let desktop_source = DesktopWindowXamlSource::new()?;

    let interop = desktop_source.cast::<IDesktopWindowXamlSourceNative>()?;

    interop.AttachToWindow(h_wnd.0 as windows::RawPtr)?;

    // This HWND will be the window handler for the XAML Island: A child window that contains XAML.
    // Get the new child window's HWND.
    let h_wnd_xaml_island = HWND(interop.get_WindowHandle()? as isize);

    // Update the XAML Island window size because initially it is 0,0.
    unsafe { SetWindowPos(h_wnd_xaml_island, HWND::NULL, 200, 100, 800, 200, SWP_SHOWWINDOW); }

    // Create the XAML content.
    let container = Controls::StackPanel::new()?;

    // let s = container.Background()?;

    // println!("{:?}", &container);

    // let path = Uri::CreateUri("ms-appx:///Page.xaml")?;
    // Application::LoadComponent(&app, path)?;

    // let grid = Controls::Grid::new()?;
    // grid.SetPadding(40.0)?;

    unsafe {
        UpdateWindow(h_wnd);

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, HWND::NULL, 0, 0).as_bool() {
            if msg.message == WM_QUIT {
                break;
            }
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    Ok(())
}

fn create_window(h_instance: HINSTANCE, ptr: *mut u16) -> windows::Result<HWND> {

    let wc = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_VREDRAW | CS_HREDRAW,
        lpfnWndProc: Some(window_proc),
        lpszClassName: PWSTR(ptr),
        hIcon: unsafe { LoadIconW(h_instance, IDI_APPLICATION) },
        hCursor: unsafe { LoadCursorW(h_instance, IDC_APPSTARTING) },
        hbrBackground: HBRUSH(15),
        ..Default::default()
    };

    unsafe {
        if RegisterClassExW(&wc) == 0 {
            return Err(windows::Error::new(
                windows::HRESULT(std::io::Error::last_os_error().raw_os_error().unwrap() as u32),
                "Failed to register Window class",
            ));
        }
        let h_wnd = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            wc.lpszClassName,
            wc.lpszClassName,
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            HWND::NULL,
            HMENU::NULL,
            h_instance,
            null_mut(),
        );

        if !h_wnd.is_null() {
            Ok(h_wnd)
        } else {
            Err(windows::Error::new(
                windows::HRESULT(std::io::Error::last_os_error().raw_os_error().unwrap() as u32),
                "Failed to create Window",
            ))
        }
    }
}

pub extern "system" fn window_proc(
    h_wnd: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    unsafe {
        match msg {
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_SIZE => {
                let mut client_rect = RECT::default();
                GetClientRect(h_wnd, &mut client_rect);
                MoveWindow(CHILD_HWND, 200, 200, 400, 500, true);
                ShowWindow(CHILD_HWND, SW_SHOW);
                LRESULT(0)
            }
            WM_CREATE => {
                let h_instance = GetModuleHandleW(PWSTR(null_mut()));
                CHILD_HWND = CreateWindowExW(
                    WINDOW_EX_STYLE(0),
                    "ChildWClass",
                    PWSTR(null_mut()),
                    WS_CHILD | WS_BORDER,
                    0,
                    0,
                    0,
                    0,
                    h_wnd,
                    HMENU::NULL,
                    h_instance,
                    null_mut(),
                );
                LRESULT(0)
            }
            _ => DefWindowProcW(h_wnd, msg, w_param, l_param),
        }
    }
}
