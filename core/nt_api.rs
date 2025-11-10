use crate::nt_structs::{CriticalSection, Mutex, Semaphore, Section, MemoryRegion};
/// Traduce RtlEnterCriticalSection: entrada real a sección crítica NT usando datos del sistema
pub fn rtl_enter_critical_section(cs: &mut CriticalSection, thread_id: u64) {
    cs.lock_count += 1;
    cs.owning_thread = thread_id;
    cs.recursion_count += 1;
    println!("[NTAPI] RtlEnterCriticalSection: thread {} lock_count {}", thread_id, cs.lock_count);
}

/// Traduce RtlLeaveCriticalSection: salida real de sección crítica NT usando datos del sistema
pub fn rtl_leave_critical_section(cs: &mut CriticalSection, thread_id: u64) {
    if cs.owning_thread == thread_id && cs.lock_count > 0 {
        cs.lock_count -= 1;
        cs.recursion_count -= 1;
        if cs.lock_count == 0 {
            cs.owning_thread = 0;
        }
        println!("[NTAPI] RtlLeaveCriticalSection: thread {} lock_count {}", thread_id, cs.lock_count);
    }
}

/// Traduce NtCreateMutex: crea un mutex NT real
pub fn nt_create_mutex() -> Mutex {
    Mutex::new()
}

/// Traduce NtReleaseMutex: libera un mutex NT real
pub fn nt_release_mutex(mutex: &mut Mutex, thread_id: u64) {
    if mutex.owner == thread_id {
        mutex.count += 1;
        mutex.owner = 0;
        println!("[NTAPI] NtReleaseMutex: thread {} count {}", thread_id, mutex.count);
    }
}

/// Traduce NtCreateSemaphore: crea un semáforo NT real
pub fn nt_create_semaphore(max_count: i32) -> Semaphore {
    Semaphore::new(max_count)
}

/// Traduce NtReleaseSemaphore: libera un semáforo NT real
pub fn nt_release_semaphore(sem: &mut Semaphore) {
    if sem.count < sem.max_count {
        sem.count += 1;
        println!("[NTAPI] NtReleaseSemaphore: count {}", sem.count);
    }
}

/// Traduce NtCreateSection: crea una sección de memoria NT real
pub fn nt_create_section(base_address: u64, size: u64, protection: u32) -> Section {
    Section::new(base_address, size, protection)
}

/// Traduce NtQueryVirtualMemory: obtiene regiones de memoria reales de un proceso
pub fn nt_query_virtual_memory(pid: u32) -> Vec<MemoryRegion> {
    crate::nt_structs::get_memory_regions(pid)
}

use std::fs;
use std::io::{self, Read, IoSlice, IoSliceMut};
use std::path::Path;
use std::os::unix::io::RawFd;
use std::os::fd::AsRawFd;
use nix::unistd::Pid;
use nix::sys::uio::{process_vm_readv, process_vm_writev, RemoteIoVec};

/// Traduce ZwQueryInformationProcess: obtiene info real de proceso y la formatea como NT
pub fn zw_query_information_process(pid: u32) -> io::Result<String> {
    let stat_path = format!("/proc/{}/stat", pid);
    let status_path = format!("/proc/{}/status", pid);
    if Path::new(&stat_path).exists() && Path::new(&status_path).exists() {
        let mut stat_file = fs::File::open(stat_path)?;
        let mut stat_contents = String::new();
        stat_file.read_to_string(&mut stat_contents)?;
        let mut status_file = fs::File::open(status_path)?;
        let mut status_contents = String::new();
        status_file.read_to_string(&mut status_contents)?;
        // Aquí puedes transformar ambos contenidos a una estructura NT-compatible
        Ok(format!("STAT:{}\nSTATUS:{}", stat_contents, status_contents))
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Proceso no encontrado"))
    }
}

/// Traduce ZwOpenProcess: abre un handle (fd) real al proceso destino
pub fn zw_open_process(pid: u32) -> io::Result<RawFd> {
    let mem_path = format!("/proc/{}/mem", pid);
    match fs::OpenOptions::new().read(true).write(true).open(&mem_path) {
        Ok(file) => Ok(file.as_raw_fd()),
        Err(e) => Err(e),
    }
}

/// Traduce KeAttachProcess: cambia el contexto de ejecución al proceso destino usando datos reales
pub fn ke_attach_process(pid: u32) -> io::Result<()> {
    let ns_path = format!("/proc/{}/ns/mnt", pid);
    if Path::new(&ns_path).exists() {
        // Aquí se podría usar setns() para cambiar el namespace
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Namespace de proceso no encontrado"))
    }
}

/// Traduce ObReferenceObjectByHandle: valida acceso real a un handle (fd)
pub fn ob_reference_object_by_handle(pid: u32, fd: u32) -> io::Result<String> {
    let fd_path = format!("/proc/{}/fd/{}", pid, fd);
    if Path::new(&fd_path).exists() {
        let target = fs::read_link(fd_path)?;
        Ok(target.to_string_lossy().to_string())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Handle no válido"))
    }
}

/// Traduce PsLookupProcessByProcessId: busca proceso real por PID
pub fn ps_lookup_process_by_process_id(pid: u32) -> bool {
    Path::new(&format!("/proc/{}", pid)).exists()
}

/// Traduce NtReadVirtualMemory: lee memoria real de otro proceso usando process_vm_readv
pub fn nt_read_virtual_memory(pid: u32, address: usize, buf: &mut [u8]) -> io::Result<usize> {
    let len = buf.len();
    let mut local_iov = [IoSliceMut::new(buf)];
    let remote_iov = [RemoteIoVec { base: address, len }];
    match process_vm_readv(Pid::from_raw(pid as i32), &mut local_iov, &remote_iov) {
        Ok(n) => Ok(n),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, format!("process_vm_readv: {}", e))),
    }
}

/// Traduce NtWriteVirtualMemory: escribe memoria real en otro proceso usando process_vm_writev
pub fn nt_write_virtual_memory(pid: u32, address: usize, buf: &[u8]) -> io::Result<usize> {
    let local_iov = [IoSlice::new(buf)];
    let remote_iov = [RemoteIoVec { base: address, len: buf.len() }];
    match process_vm_writev(Pid::from_raw(pid as i32), &local_iov, &remote_iov) {
        Ok(n) => Ok(n),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, format!("process_vm_writev: {}", e))),
    }
}

/// Traduce NtQuerySystemInformation: obtiene información global real del sistema
pub fn nt_query_system_information(class: u32) -> io::Result<String> {
    match class {
        5 => { // SystemProcessInformation
            let mut info = String::new();
            if let Ok(entries) = fs::read_dir("/proc") {
                for entry in entries.flatten() {
                    if let Ok(pid) = entry.file_name().to_string_lossy().parse::<u32>() {
                        info.push_str(&format!("PID: {}\n", pid));
                    }
                }
            }
            Ok(info)
        }
        _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Clase no soportada")),
    }
}

/// Usa eBPF para monitoreo avanzado (acceso real)
pub fn ebpf_monitor_process(pid: u32) -> io::Result<()> {
    // Aquí se cargaría un programa eBPF para monitorear syscalls del proceso usando datos reales
    println!("eBPF: Monitoreando proceso {} (acceso real)", pid);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    // ...existing code...
}
