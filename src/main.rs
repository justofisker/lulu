use ash::{vk, Entry};
use std::ffi::{CStr, CString};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[allow(dead_code)]
struct State {
    entry: Entry,
    instance: ash::Instance,
    physical_device: vk::PhysicalDevice,
    window_size: vk::Extent2D,
    surface: vk::SurfaceKHR,
    device: ash::Device,
    queue: vk::Queue,
    debug_callback: vk::DebugUtilsMessengerEXT,
}

impl State {
    pub fn new(window: &Window) -> Self {
        let entry = Entry::linked();

        let app_info = vk::ApplicationInfo {
            api_version: vk::API_VERSION_1_0,
            ..Default::default()
        };

        let mut surface_extensions = ash_window::enumerate_required_extensions(&window)
            .unwrap()
            .to_vec();

        surface_extensions.push(ash::extensions::ext::DebugUtils::name().as_ptr());

        let debug_layers = vec![CString::new("VK_LAYER_KHRONOS_validation").unwrap()];

        let debug_layers: Vec<*const i8> = debug_layers
            .iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();

        let instance_create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            pp_enabled_extension_names: surface_extensions.as_ptr(),
            enabled_extension_count: surface_extensions.len() as u32,
            pp_enabled_layer_names: debug_layers.as_ptr(),
            enabled_layer_count: debug_layers.len() as u32,
            ..Default::default()
        };

        let instance = unsafe { entry.create_instance(&instance_create_info, None) }.unwrap();

        let debug_utils = ash::extensions::ext::DebugUtils::new(&entry, &instance);
        let debug_create_info = vk::DebugUtilsMessengerCreateInfoEXT {
            message_severity: vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
            message_type: vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
            pfn_user_callback: Some(debug_report_callback),
            ..Default::default()
        };

        let debug_callback = unsafe { debug_utils.create_debug_utils_messenger(&debug_create_info, None) }.unwrap();

        let surface =
            unsafe { ash_window::create_surface(&entry, &instance, &window, None) }.unwrap();

        let physical_device = unsafe { instance.enumerate_physical_devices() }.unwrap()[0];

        let queues =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        let queue_family_index = queues
            .iter()
            .position(|queue| queue.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .expect("Failed to find graphics queue") as u32;

        let queue_prorities = 1.0 as f32;

        let queue_create_info = vk::DeviceQueueCreateInfo {
            queue_family_index: queue_family_index,
            queue_count: 1,
            p_queue_priorities: &queue_prorities,
            ..Default::default()
        };

        let device_create_info = vk::DeviceCreateInfo {
            p_queue_create_infos: &queue_create_info,
            queue_create_info_count: 1,
            ..Default::default()
        };

        let device =
            unsafe { instance.create_device(physical_device, &device_create_info, None) }.unwrap();

        let queue = unsafe { device.get_device_queue(queue_family_index, 0) };

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
            device,
            queue,
            debug_callback,
        }
    }
}

impl Drop for State {
    fn drop(&mut self) {
        // TODO: figure out how to get this actually called
        println!("bye!");
    }
}

#[allow(unused_variables)]
unsafe extern "system" fn debug_report_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut std::ffi::c_void,
) -> vk::Bool32 {
    let message = CStr::from_ptr((*p_callback_data).p_message);
    let severity = format!("{:?}", message_severity).to_lowercase();
    let ty = format!("{:?}", message_type).to_lowercase();
    println!("[Debug][{}][{}] {:?}", severity, ty, message);
    vk::FALSE
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
