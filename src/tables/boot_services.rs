use core::{ffi::c_void, mem::MaybeUninit, ptr::null_mut};

use crate::{protocols::DevicePath, tables::TableHeader, Guid, Handle, Protocol, Result, Status};

use super::Signature;

// FIXME: use wrapper structs for ty
// FIXME: Make sure the pointers have the correct mutability
#[repr(C)]
pub struct BootServices {
    hdr: TableHeader,
    raise_tpl: unsafe extern "efiapi" fn(new_tpl: Tpl) -> Tpl,
    restore_tpl: unsafe extern "efiapi" fn(old_tpl: Tpl),
    allocate_pages: unsafe extern "efiapi" fn(
        allocate_type: AllocateType,
        memory_type: MemoryType,
        pages: usize,
        memory: *mut PhysicalAddress,
    ) -> Status,
    free_pages: unsafe extern "efiapi" fn(memory: PhysicalAddress, pages: usize) -> Status,
    get_memory_map: unsafe extern "efiapi" fn(
        memory_map_size: usize,
        memory_map: *mut MemoryDescriptor,
        map_key: *mut usize,
        descriptor_size: *mut usize,
        descriptor_version: *mut u32,
    ) -> Status,
    allocate_pool: unsafe extern "efiapi" fn(
        pool_type: MemoryType,
        size: usize,
        buffer: *mut *mut c_void,
    ) -> Status,
    free_pool: unsafe extern "efiapi" fn(buffer: *mut c_void) -> Status,
    create_event: unsafe extern "efiapi" fn(
        ty: u32,
        notify_tpl: Tpl,
        notify_function: Option<EventNotify>,
        notify_context: *mut c_void,
        event: *mut Event,
    ) -> Status,
    /// FIXME: implement EFI_TIMER_DELAY
    set_timer: unsafe extern "efiapi" fn(efi_event: Event, ty: u32, trigger_time: u64) -> Status,
    wait_for_event: unsafe extern "efiapi" fn(
        number_of_events: usize,
        event: *mut Event,
        index: *mut usize,
    ) -> Status,
    signal_event: unsafe extern "efiapi" fn(event: Event) -> Status,
    close_event: unsafe extern "efiapi" fn(event: Event) -> Status,
    check_event: unsafe extern "efiapi" fn(event: Event) -> Status,
    /// FIXME: implement EFI_INTERFACE_TYPE
    install_protocol_interface: unsafe extern "efiapi" fn(
        handle: *mut Handle,
        protocol: *mut Guid,
        interface_type: u32,
        interface: *const c_void,
    ) -> Status,
    reinstall_protocol_interface: unsafe extern "efiapi" fn(
        handle: Handle,
        protocol: *mut Guid,
        old_interface: *const c_void,
        new_interface: *const c_void,
    ) -> Status,
    uninstall_protocol_interface: unsafe extern "efiapi" fn(
        handle: Handle,
        protocol: *mut Guid,
        interface: *const c_void,
    ) -> Status,
    handle_protocol: unsafe extern "efiapi" fn(
        handle: Handle,
        protocol: *mut Guid,
        interface: *mut *mut c_void,
    ) -> Status,
    reserved: *mut c_void,
    register_protocol_notify: unsafe extern "efiapi" fn(
        protocol: *mut Guid,
        event: Event,
        registration: *mut *mut c_void,
    ) -> Status,
    locate_handle: unsafe extern "efiapi" fn(
        search_type: u32,
        protocol: *mut Guid,
        search_key: *mut c_void,
        buffer_size: usize,
        buffer: *mut Handle,
    ) -> Status,
    locate_device_path: unsafe extern "efiapi" fn(
        protocol: *mut Guid,
        device_path: *mut DevicePath,
        device: *mut Handle,
    ) -> Status,
    install_configuration_table:
        unsafe extern "efiapi" fn(guid: *mut Guid, table: *mut c_void) -> Status,
    load_image: unsafe extern "efiapi" fn(
        boot_policy: bool,
        parent_image_handle: Handle,
        device_path: *const DevicePath,
        source_buffer: *mut c_void,
        source_size: usize,
        image_handle: *mut Handle,
    ) -> Status,
    start_image: unsafe extern "efiapi" fn(
        image_handle: Handle,
        exit_data_size: *mut usize,
        exit_data: *mut *mut u16,
    ) -> Status,
    exit: unsafe extern "efiapi" fn(
        image_handle: Handle,
        exit_status: Status,
        exit_data_size: usize,
        exit_data: *mut u16,
    ) -> Status,
    unload_image: unsafe extern "efiapi" fn(image_handle: Handle) -> Status,
    exit_boot_services: unsafe extern "efiapi" fn(image_handle: Handle, map_key: usize) -> Status,
    get_next_monotic_count: unsafe extern "efiapi" fn(count: *mut u64) -> Status,
    stall: unsafe extern "efiapi" fn(microseconds: usize) -> Status,
    set_watchdog_timer: unsafe extern "efiapi" fn(
        timeout: usize,
        watchdog_code: u64,
        data_size: usize,
        watchdog_data: *const u16,
    ) -> Status,
    connect_controller: unsafe extern "efiapi" fn(
        controller_handle: Handle,
        driver_image_handle: *mut Handle,
        remaining_device_path: *const DevicePath,
        recursive: bool,
    ) -> Status,
    disconnect_controller: unsafe extern "efiapi" fn(
        controller_handle: Handle,
        driver_image_handle: Handle,
        child_handle: Handle,
    ) -> Status,
    open_protocol: unsafe extern "efiapi" fn(
        handle: Handle,
        protocol: *const Guid,
        interface: *mut *mut c_void,
        agent_handle: Handle,
        controller_handle: Handle,
        attributes: OpenProtocolAttributes,
    ) -> Status,
    close_protocol: unsafe extern "efiapi" fn(
        handle: Handle,
        protocol: *mut Guid,
        agent_handle: Handle,
        controller_handle: Handle,
    ) -> Status,
    open_protocol_information: unsafe extern "efiapi" fn(
        handle: Handle,
        protocol: *mut Guid,
        entry_buffer: *mut *mut OpenProtocolInformationEntry,
        entry_count: *mut usize,
    ) -> Status,
    protocols_per_handle: unsafe extern "efiapi" fn(
        handle: Handle,
        protocol_buffer: *mut *mut *mut Guid,
        protocol_buffer_count: *mut usize,
    ) -> Status,
    locate_handle_buffer: unsafe extern "efiapi" fn(
        search_type: u32,
        protocol: *mut Guid,
        search_key: *mut c_void,
        no_handles: *mut usize,
        buffer: *mut *mut Handle,
    ) -> Status,
    locate_protocol: unsafe extern "efiapi" fn(
        protocol: *mut Guid,
        registration: *mut c_void,
        interface: *mut *mut c_void,
    ) -> Status,
    install_multiple_protocol_interfaces:
        unsafe extern "efiapi" fn(handle: *mut Handle, ...) -> Status,
    uninstall_multiple_protocol_interfaces:
        unsafe extern "efiapi" fn(handle: Handle, ...) -> Status,
    calculate_crc32:
        unsafe extern "efiapi" fn(data: *mut c_void, data_size: usize, crc32: *mut u32) -> Status,
    copy_mem:
        unsafe extern "efiapi" fn(destination: *mut c_void, source: *mut c_void, length: usize),

    set_mem: unsafe extern "efiapi" fn(buffer: *mut c_void, size: usize, value: u8),
    create_event_ex: unsafe extern "efiapi" fn(
        ty: u32,
        notify_tpl: Tpl,
        notify_function: Option<EventNotify>,
        notify_context: *mut c_void,
        event_grou: *mut Guid,
        event: *mut Event,
    ) -> Status,
}

#[repr(transparent)]
pub struct Tpl(usize);

impl Tpl {
    pub const APPLICATION: Self = Self(4);
    pub const CALLBACK: Self = Self(8);
    pub const NOTIFY: Self = Self(16);
    pub const HIGH_LEVEL: Self = Self(31);
}

#[repr(transparent)]
pub struct MemoryType(u32);

impl MemoryType {
    pub const RESERVED_MEMORY_TYPE: Self = Self(0);
    pub const LOADER_CODE: Self = Self(1);
    pub const LOADER_DATA: Self = Self(2);
    pub const BOOT_SERVICES_CODE: Self = Self(3);
    pub const BOOT_SERVICES_DATA: Self = Self(4);
    pub const RUNTIME_SERVICES_CODE: Self = Self(5);
    pub const RUNTIME_SERVICES_DATA: Self = Self(6);
    pub const CONVENTIONAL_MEMORY: Self = Self(7);
    pub const UNUSABLE_MEMORY: Self = Self(8);
    pub const ACPIRECLAIM_MEMORY: Self = Self(9);
    pub const ACPIMEMORY_NVS: Self = Self(10);
    pub const MEMORY_MAPPED_IO: Self = Self(11);
    pub const MEMORY_MAPPED_IOPORT_SPACE: Self = Self(12);
    pub const PAL_CODE: Self = Self(13);
    pub const PERSISTENT_MEMORY: Self = Self(14);
    pub const UNACCEPTED_MEMORY_TYPE: Self = Self(15);
    pub const MAX_MEMORY_TYPE: Self = Self(16);
}

#[repr(transparent)]
pub struct AllocateType(u32);

impl AllocateType {
    const ALLOCATE_ANY_PAGES: Self = Self(0);
    const ALLOCATE_MAX_ADDRESS: Self = Self(1);
    const ALLOCATE_ADDRESS: Self = Self(2);
    const MAX_ALLOCATE_TYPE: Self = Self(3);
}

#[repr(transparent)]
pub struct PhysicalAddress(pub u64);

impl From<u64> for PhysicalAddress {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

pub struct VirtualAddress(u64);

pub struct MemoryDescriptor {
    ty: u32,
    physical_start: PhysicalAddress,
    virtual_start: VirtualAddress,
    number_of_pages: u64,
    attribute: u64,
}

pub type Event = *mut c_void;
pub type EventNotify = unsafe extern "efiapi" fn(event: Event, context: *mut c_void);

// //EFI_TIMER_DELAY
// //******************************************************
// typedef enum {
//     TimerCancel,
//     TimerPeriodic,
//     TimerRelative
//     } EFI_TIMER_DELAY;

#[repr(transparent)]
pub struct OpenProtocolAttributes(u32);

impl OpenProtocolAttributes {
    const BY_HANDLE_PROTOCOL: Self = Self(0x00000001);
    const GET_PROTOCOL: Self = Self(0x00000002);
    const TEST_PROTOCOL: Self = Self(0x00000004);
    const BY_CHILD_CONTROLLER: Self = Self(0x00000008);
    const BY_DRIVER: Self = Self(0x00000010);
    const BY_DRIVER_EXCLUSIVE: Self = Self(Self::BY_DRIVER.0 | Self::EXCLUSIVE.0);
    const EXCLUSIVE: Self = Self(0x00000020);
}

#[repr(C)]
pub struct OpenProtocolInformationEntry {
    pub agent_handle: Handle,
    pub controller_handle: Handle,
    pub attributes: OpenProtocolAttributes,
    pub open_count: u32,
}

impl BootServices {
    pub fn signature(&self) -> Signature {
        self.hdr.signature
    }

    // FIXME: check for errors
    pub fn allocate_pool(&self, memory_type: MemoryType, size: usize) -> Result<*mut c_void> {
        let mut buffer = null_mut();
        unsafe { (self.allocate_pool)(memory_type, size, &mut buffer) }.as_result_with(buffer)
    }

    pub fn free_pool(&self, buffer: *mut c_void) -> Result {
        unsafe { (self.free_pool)(buffer) }.as_result()
    }

    pub fn allocate_pages_at_address(
        &self,
        memory_type: MemoryType,
        pages: usize,
        address: PhysicalAddress,
    ) -> Result {
        let mut memory = address;

        unsafe {
            (self.allocate_pages)(
                AllocateType::ALLOCATE_ADDRESS,
                memory_type,
                pages,
                &mut memory,
            )
        }
        .as_result()
    }

    pub fn allocate_any_pages(
        &self,
        memory_type: MemoryType,
        pages: usize,
    ) -> Result<PhysicalAddress> {
        let mut address = PhysicalAddress::from(0);

        unsafe {
            (self.allocate_pages)(
                AllocateType::ALLOCATE_ANY_PAGES,
                memory_type,
                pages,
                &mut address,
            )
        }
        .as_result_with(address)
    }

    pub fn open_protocol<P: Protocol>(&self, handle: &Handle, agent: &Handle) -> Result<&P> {
        let mut protocol = MaybeUninit::<*mut P>::uninit();
        let status = unsafe {
            (self.open_protocol)(
                *handle,
                &P::GUID,
                protocol.as_mut_ptr().cast(),
                *agent,
                Handle::null(),
                OpenProtocolAttributes::BY_HANDLE_PROTOCOL,
            )
        };

        status.as_result_with(unsafe { &*protocol.assume_init() })
    }

    // #[allow(unused)]
    // pub fn load_image(
    //     &self,
    //     boot_policy: bool,
    //     parent_image_handle: &Handle,
    //     buf: &[u8],
    // ) -> Result<Handle> {
    //     let mut image_handle = Handle::null();

    //     let device_path =
    //         self.open_protocol::<DevicePath>(&parent_image_handle, &parent_image_handle)?;

    //     unsafe {
    //         (self.load_image)(
    //             false,
    //             *parent_image_handle,
    //             device_path,
    //             buf.as_ptr() as _,
    //             buf.len(),
    //             &mut image_handle,
    //         )
    //     }
    //     .as_result_with(image_handle)
    // }

    pub fn start_image(&self, image_handle: Handle) -> Result {
        unsafe { (self.start_image)(image_handle, null_mut(), null_mut()) }.as_result()
    }
}
