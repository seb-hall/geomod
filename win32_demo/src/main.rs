use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::*,
    Win32::{System::LibraryLoader::GetModuleHandleA, Graphics::Gdi::BeginPaint}, Win32::UI::WindowsAndMessaging::*,
    Win32::Graphics::OpenGL::*,
};


use gl::*;

struct windowInfo {

}


fn main() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleA(None)?;
        debug_assert!(instance.0 != 0);

        let window_class = s!("window");

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: instance,
            lpszClassName: window_class,

            style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        let window: HWND = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class,
            s!("This is a sample window"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            750,
            500,
            None,
            None,
            instance,
            None,
        );

        let mut pfd: PIXELFORMATDESCRIPTOR = PIXELFORMATDESCRIPTOR {
            nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16,
            nVersion: 1,
            dwFlags: PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER | PFD_DRAW_TO_WINDOW,
            iPixelType: PFD_TYPE_RGBA,
            cColorBits: 24,
            cDepthBits: 32,
            ..Default::default()
        };

        let hdc = GetDC(window);

        let pixelFormat = ChoosePixelFormat(hdc, &mut pfd);

        SetPixelFormat(hdc, pixelFormat, &mut pfd);

        let renderContext = wglCreateContext(hdc);
        wglMakeCurrent(hdc, renderContext.unwrap());



        let mut message = MSG::default();

        //while GetMessageA(&mut message, None, 0, 0).into() {

        let mut should_quit: bool = false;

        while (!should_quit) {

            if bool::from(PeekMessageA(&mut message, window, 0, 0, PM_REMOVE)) {
                if (message.message == WM_QUIT) {
                    should_quit = true;
                }

                DispatchMessageA(&message);
            }
        }

        Ok(())
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                //println!("WM_PAINT");
                /*let mut ps: PAINTSTRUCT = PAINTSTRUCT { ..Default::default() };
                let hdc = BeginPaint(window, &mut ps);
                let message = String::from("Hello, Windows!");
                let mut rect: RECT = RECT { ..Default::default() };
                GetClientRect(window, &mut rect);

                //let brush: HBRUSH = CreateSolidBrush(color as COLORREF);
                //FillRect(hdc,rect, &mut brush);
                TextOutA(hdc, 0, 0, &message.as_bytes());
                EndPaint(window, &mut ps);*/

                let hdc =  GetDC(window);

                let mut rect: RECT = RECT { ..Default::default() };
                GetClientRect(window, &mut rect);

                glViewport(0, 0, rect.right, rect.bottom);
                glMatrixMode(GL_PROJECTION);
                glLoadIdentity();
                gluPerspective(70.0, rect.right as f64/rect.bottom as f64, 0.0, 1000.0);
                glMatrixMode(GL_MODELVIEW);
                glLoadIdentity();
                gluLookAt(0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
                glClearColor(0.2, 0.2, 0.2, 0.2);
                glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);


                glBegin(GL_TRIANGLES);
                glColor3f(1.0, 0.0, 0.0);
                glVertex3f(-5.0, -4.0, 0.0);
                glColor3f(0.0, 1.0, 0.0);
                glVertex3f(5.0, -4.0, 0.0);
                glColor3f(0.0, 0.0, 1.0);
                glVertex3f(0.0, 3.5, 0.0);
                glEnd();

                SwapBuffers(hdc);
                ValidateRect(window, None);
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}