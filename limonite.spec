Name:		limonite
Version:	0.0.1
Release:	1%{?dist}
Summary: meh

Group:	Applications/Publishing
License: Apache 2
URL: http://github.com/qmx/limonite
Source0:	https://github.com/qmx/limonite/archive/limonite-%{version}-%{release}.tar.gz

BuildRequires:	rust-binary

%description


%prep
%setup -q


%build
cargo build

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}%{_bindir}/
cp -p target/debug/limonite %{buildroot}%{_bindir}/

%files
/usr/bin/limonite
%changelog
* Thu Jan 07 2016 Douglas Campos <qmx@qmx.me> 0.0.1-1
- new package built with tito


