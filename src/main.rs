#![windows_subsystem = "windows"]

use bindings::{
    Microsoft,
    Microsoft::UI::Xaml::{
        Application, ApplicationInitializationCallback, Window,LaunchActivatedEventArgs
    },
    Microsoft::UI::Dispatching::{DispatcherQueue,DispatcherQueueController},
    Windows::Win32::{
        Foundation::{HWND, RECT},
        UI::{
            WindowsAndMessaging::{
                GetSystemMetrics, GetWindowRect, SetWindowPos, SM_CXSCREEN, SM_CYSCREEN,
                SWP_NOMOVE, SWP_NOSIZE,
            },
        },
    },
};

use windows::{implement};

#[implement(extend Microsoft::UI::Xaml::Application, override OnLaunched)]
struct App {
    _window: Option<Window>,
}

#[allow(non_snake_case)]
impl App {
    fn OnLaunched(&mut self, _: &Option<LaunchActivatedEventArgs>) -> windows::Result<()> {
        let window = Window::new()?;
        window.SetTitle("WinUI Desktop, Unpackaged (Rust)")?;
        window.Activate();
        self._window = Some(window.clone());
        //uncomment this line to abandon the event pump
        //This will cause resizing to work reliably again
        // return windows::Result::Ok(());

        // Ensure we have a DispatcherQueue on this thread.
        // Note - this pattern is only necessary if you need to create a DispatcherQueue on a thread (ex as part
        // of initialization). For the vast majority of scenarios, a hosting framework has already created one
        // on your behalf and so you should simply call DispatcherQueue.GetForCurrentThread() to retrieve it.

        if DispatcherQueue::GetForCurrentThread().ok() == None
        {
            println!("Making new dispatcher queue");
            DispatcherQueueController::CreateOnCurrentThread();
        }

        let hwnd = windows_app::window_handle(&window.clone().into()).unwrap();
        loop {
            let mut msg = bindings::Windows::Win32::UI::WindowsAndMessaging::MSG::default();
            let foundation_hwnd = bindings::Windows::Win32::Foundation::HWND(hwnd);
            let ret = unsafe{ bindings::Windows::Win32::UI::WindowsAndMessaging::GetMessageW(&mut msg, foundation_hwnd, 0,0)};
            if ret.0 == 1 {
                //" If your application must obtain character input from the user, include the TranslateMessage function in the loop. TranslateMessage translates virtual-key messages into character messages. The following example shows the message loop in the WinMain function of a simple Windows-based application."
                //https://docs.microsoft.com/en-us/windows/win32/winmsg/using-messages-and-message-queues
                unsafe{ bindings::Windows::Win32::UI::WindowsAndMessaging::TranslateMessage(&msg)};
                // println!("seen message {}",msg.message);
                unsafe{ bindings::Windows::Win32::UI::WindowsAndMessaging::DispatchMessageW(&msg)} ;
            }
            else {
                break;
            }

        }
       windows::Result::Ok(())
    }
}

fn main() -> windows::Result<()> {
    windows_app::bootstrap::initialize().and_then(|_| {
        Application::Start(ApplicationInitializationCallback::new(|_| {
            App { _window: None }.new()?;
            Ok(())
        }))
    })
}
