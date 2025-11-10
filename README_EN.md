# KernelBridge: Logbook, Guide and Complete Manual

## 🎯 Purpose and Vision

KernelBridge is an experimental system and reference platform for:
- Running Windows games on Linux with advanced anti-cheats (ACE, EAC, etc.)
- Exposing real NT telemetry and structures to anti-cheats from Linux
- Integrating TPM, IMA/EVM, AppArmor/SELinux and integrity monitoring
- Automating configuration, optimization and troubleshooting of complex games
- Serving as a base for research, development and documentation in the Linux gaming community

---

## 🧭 Logbook, Experiments and Real Obstacles

---

## 🏗️ Architecture, Modules and Technical Limits

### What is KernelBridge?
Allows you to run Windows games on Linux while respecting the real environment (Steam/Proton, Bottles, Lutris) and exposing "NT-style" telemetry (NT Device Proxy) from a secure daemon. It does not emulate or bypass anti-cheats: it exposes real, not simulated, data.

### Main Components
- **GUI (GTK4 + Relm4):** Lists games, launches with the correct runtime, shows logs, allows NT Proxy diagnostics.
- **Daemon (Rust):** Background service, manages modules, exposes NT Device Proxy, checks TPM, runs games in isolated environments.
- **Core (Rust/C):** Hybrid NT APIs, exposes NT structures and syscalls using real Linux data.

### Key Modules
- **tpm_manager:** TPM detection and attestation.
- **integrity_monitor:** IMA/EVM, AppArmor, SELinux state, hashes and signatures.
- **anti_cheat_gateway:** Exposes simulated NT structures with real data, responds to anti-cheats without hiding the environment.
- **game_launcher:** Launches games after verifying requirements, prepares a secure environment and reports status/logs.
- **event_broker, memory_auditor, kernel_validator, system_bridge_api:** Metrics, monitoring, kernel validation, system extensions.
- **nt_device_proxy:** Translates NT IOCTLs to real Linux data (processes, threads, modules, memory, handles, TPM attestation, sandbox/VM detection, etc.).

### Flows and Communication
- GUI ↔ Daemon: UNIX socket (`/tmp/kernelbridge.sock`).
- Daemon ↔ Core: FFI, sockets or direct calls.
- Modules use ptrace, process_vm_readv/writev, eBPF, TPM, IMA/EVM, AppArmor/SELinux.
- Each game runs in its own namespace (PID, mount, net, etc.) for isolation and traceability.

### Security and Real Limits
- Principle of least privilege: the GUI never injects or modifies processes, the daemon does not bypass anti-cheats.
- No simulation or bypass: all access and reporting is legitimate and verifiable.
- If the anti-cheat requires kernel drivers (ACE), there is no solution on Linux: neither emulation, reverse engineering, nor virtualization will solve it.

### Example Commands and Real Telemetry
```bash
# Query processes via NT Device Proxy
printf 'NT_IOCTL:GET_PROCESS_LIST\n' | socat - UNIX-CONNECT:/tmp/kernelbridge.sock
# Query threads of a PID
printf 'NT_IOCTL:GET_THREAD_LIST:1234\n' | socat - UNIX-CONNECT:/tmp/kernelbridge.sock
# Attestation state
printf 'NT_IOCTL:GET_ATTESTATION\n' | socat - UNIX-CONNECT:/tmp/kernelbridge.sock
```

### Simulated NT Structures (with real data)
- **EPROCESS:** PID, name, PPID, threads, memory, state (from /proc)
- **ETHREAD:** TID, PID, state (from /proc)
- **HANDLE_TABLE:** Open handles (from /proc/[pid]/fd)
- **OBJECT_HEADER:** Type, handle_count, flags

### Implemented NT APIs
- ZwQueryInformationProcess, KeAttachProcess, ObReferenceObjectByHandle, PsLookupProcessByProcessId, ZwOpenProcess, NtReadVirtualMemory/NtWriteVirtualMemory (translated to /proc, ptrace, process_vm_readv/writev)

### Integrity and TPM
- Validation of kernel and critical binaries using TPM, IMA/EVM, Secure Boot.
- If there is no TPM, games that require it will not launch.

### Roadmap and Status
- Base infrastructure, hybrid NT APIs, anti-cheat integration, secure launcher, advanced GUI, Flatpak/AppImage packaging: **completed**.
- Additional modules, metrics, kernel validation, launcher integration, documentation and scripts: **completed**.

### Real Limits and Challenges
- **ACE (AntiCheatExpert):** Incompatible with Linux/Proton due to requiring kernel-level drivers, digital signatures and validations impossible to emulate. No bypass, emulation or translation is technically or legally viable.
- **EAC (EasyAntiCheat):** Compatible with Proton in permissive mode. Scripts and launch options automate its use when possible.
- **Virtualization:** ACE detects VMs, poor performance, not a real solution.
- **Reverse engineering:** Illegal, extreme obfuscation, real legal risk.
- **Dual boot:** Only real solution for games with mandatory ACE.

---

## 🧭 Logbook, Experiments and Real Obstacles

### 1. The Dream: Playing Delta Force with ACE on Linux

The goal was clear: to get Delta Force, with its ACE (kernel-level) anti-cheat, running on Linux using Steam Flatpak and Proton. The process was a mix of hope, creativity, frustration and learning.

---

### 2. First Attempts: Emulation and Scripts

- Tried to emulate Windows drivers (.sys) in Wine: impossible, Wine only runs userspace code, never kernel drivers.
- Created scripts to clean ACE and force EAC, hoping the game would accept only EasyAntiCheat (EAC), which does work on Linux.
- Automated everything: driver backup, registry cleaning, launch option configuration, Steam Flatpak integration.

**Result:** EAC works in some games, but Delta Force strictly requires ACE.

---

### 3. Overflowing Creativity: What if I make my own Windows kernel?

Explored the (absurd but honest) idea of creating an open source Windows kernel for Linux:

- 20 years of development, hundreds of millions of dollars, thousands of NT functions and structures, HAL, subsystems, drivers, QA, legal, patents…
- Even if achieved, ACE would detect it's not the original kernel (digital signature, hash, communication with Tencent servers) and would ban anyway.

**Moral:** Sometimes the most complicated solution is not the best. Dual boot with Windows takes 2 hours and works 100%.

---

### 4. Ingenuity, Frustration and Technical Reality

- Considered Linux kernel modules to emulate ACE: impossible, closed code, integrity checks, encrypted communication with servers.
- Considered reverse engineering: illegal, extreme obfuscation, years of work, real legal risk.
- Considered virtualization with GPU passthrough: ACE detects virtualization, poor performance, easier to dual boot.
- Tried to "fake" ACE's heartbeat: unknown protocol, encrypted, instant ban.

**Result:** No bypass, emulation or translation is technically or legally viable for ACE.

---

### 5. What DOES work and the real contribution

- Scripts to clean ACE and enable EAC (clean_ace.sh)
- Automation of launch options, Flatpak integration, AMD optimizations
- Exhaustive documentation of every attempt, error and lesson
- Honest reflection: the limits of anti-cheat compatibility on Linux

---

### 6. Moral for the Community

> "Sometimes the most complicated solution is not the best. And sometimes, it doesn't even work."

**Lesson:**
- If your game requires ACE, the only real solution is dual boot with Windows.
- If it accepts EAC, you can play on Linux with Proton.
- Document your experiments, share your scripts and help the community avoid wasting time on impossible paths.

---

---

## 🧠 Final Reflection, Moral and Credits

### Lessons and Real Limits
- Not everything is possible on Linux: kernel-level anti-cheats like ACE are designed to be impossible to emulate, translate or bypass without official collaboration.
- Honest documentation of every attempt, error and obstacle saves others time and frustration.
- Dual boot remains the only real solution for games with mandatory ACE.
- The community advances when both successes and failures are shared.

### Moral for Future Users and Developers
> "Don't waste months on the impossible. Document, share, and help others not repeat the same mistakes. Collective knowledge is more valuable than any temporary hack."
- To those reading this looking for a real answer, even if it's harsh: here it is, without embellishment.

### Motivation
This project exists so that the next person who tries the same has a complete, honest and useful reference. If you manage to go one step further, document and share. That's how community is built.

---

**Your effort and curiosity can inspire the whole community!**

---

## 🧑‍💻 Honest Reflection and Real Learnings

This project was, above all, an exercise in honesty and technical humility. I learned that, although passion and curiosity can take you far, there are technical, legal and practical limits that cannot be forced. Trying to make kernel-level anti-cheats work on Linux is not only frustrating, but teaches you to value the work of others, respect the rules of the game, and accept that not everything is possible, no matter how hard you try.

Failing in the attempt is not a waste of time: it is true learning. Documenting every error, every obstacle and every limit is the best way to help others and to grow as a developer and as a person. If this README helps someone else not repeat the same mistakes, or helps a company understand the human and technical side of the community, then all the effort will have been worth it.

I do not seek recognition or problems, only to record what is possible and what is not, and that technical honesty is the greatest contribution we can make.

---

## 🛡️ Legal Notice, Disclaimer and Personal Reflection

This project is for educational, documentation and technical experimentation purposes only. It does not promote, facilitate or encourage bypassing, evasion, reverse engineering or breaching security systems, anti-cheat or proprietary software. All information, scripts and examples presented here are intended for legitimate interoperability, compatibility research and technical transparency in Linux environments.

**This project must not be used to violate terms of service, software licenses, or for activities that break the law or user agreements of games or platforms.**

The author and contributors are not responsible for misuse of the information or software published here. Each user is responsible for complying with local laws and the terms of the services and products they use.

**This experiment was born out of frustration and technical curiosity, not for profit or to harm anyone. I deeply respect the work of game and anti-cheat developers, and recognize the enormous challenges of computer security. If any company, publisher or developer has questions, concerns or believes something here may cause a problem, please contact before taking any action. I am willing to talk and clarify any misunderstanding.**

This project is also a testimony to the limits, frustrations and learnings of trying something difficult on Linux. I do not seek conflict, only to share what I have learned so that others do not waste time or get into trouble.

---

*Last update: November 10, 2025*
