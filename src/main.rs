use ash::{vk, Entry};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

struct State {
    entry: Entry,
    instance: ash::Instance,
    physical_device: vk::PhysicalDevice,
    window_size: vk::Extent2D,
    surface: vk::SurfaceKHR,
}

impl State {
    pub fn new(window: &Window) -> Self {
        let entry = Entry::linked();

        let app_info = vk::ApplicationInfo {
            api_version: vk::API_VERSION_1_0,
            ..Default::default()
        };

        let surface_extensions = ash_window::enumerate_required_extensions(&window).unwrap();

        let instance_create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            pp_enabled_extension_names: surface_extensions.as_ptr(),
            enabled_extension_count: surface_extensions.len() as u32,
            ..Default::default()
        };

        let instance = unsafe { entry.create_instance(&instance_create_info, None) }.unwrap();

        let surface =
            unsafe { ash_window::create_surface(&entry, &instance, &window, None) }.unwrap();

        let physical_device = unsafe { instance.enumerate_physical_devices() }.unwrap()[0];

        let queues = unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        let queue_index = queues.iter().position(|queue| {
            queue.queue_flags.contains(vk::QueueFlags::GRAPHICS)
        }).expect("Failed to find graphics queue");

        let queue_create_info = vk::DeviceQueueCreateInfo {
            queue_family_index: queue_index as u32,
            queue_count: 1,
            ..Default::default()
        };

        let device_create_info = vk::DeviceCreateInfo {
            p_queue_create_infos: &queue_create_info,
            queue_create_info_count: 1,
            pp_enabled_extension_names: surface_extensions.as_ptr(),
            enabled_extension_count: surface_extensions.len() as u32,
            ..Default::default()
        };

        //let device = unsafe { instance.create_device(physical_device, &device_create_info, None) }.unwrap();

        let size = window.inner_size();

        State {
            entry,
            instance,
            physical_device,
            window_size: vk::Extent2D {
                width: size.width,
                height: size.height,
            },
            surface,
        }
    }
}

impl Drop for State {
    fn drop(&mut self) {
        // TODO: figure out how to get this actually called
        println!("bye!");
    }
}

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(1280, 720))
        .with_title("Lulu")
        .build(&event_loop)
        .unwrap();

    let mut state = State::new(&window);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        let _ = state;

        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => {
                    state.window_size.width = size.width;
                    state.window_size.height = size.height;
                }
                _ => (),
            },
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                println!("hiya!");
            }
            _ => (),
        }
    });
}
