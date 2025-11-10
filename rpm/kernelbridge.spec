Name:           kernelbridge
Version:        0.1.0
Release:        1%{?dist}
Summary:        KernelBridge - Compatibilidad NT para juegos en Linux

License:        GPL-3.0
URL:            https://github.com/kernelbridge/kernelbridge
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  cargo, gtk4-devel, libadwaita-devel, tpm2-tools
Requires:       tpm2-tools, gtk4, libadwaita

%description
KernelBridge permite que juegos con anti-cheat funcionen en Linux
usando APIs híbridas y TPM 2.0 real.

%prep
%setup -q

%build
cargo build --release

%install
mkdir -p %{buildroot}%{_bindir}
install -m 755 target/release/kernelbridge-daemon %{buildroot}%{_bindir}/
install -m 755 target/release/kernelbridge-gui %{buildroot}%{_bindir}/

%files
%{_bindir}/kernelbridge-daemon
%{_bindir}/kernelbridge-gui

%changelog
* Wed Oct 29 2025 KernelBridge Team <kernelbridge@example.com> - 0.1.0-1
- Initial package