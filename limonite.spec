Name:		limonite
Version:	0.0.4
Release:	2%{?dist}
Summary: meh

Group:	Applications/Publishing
License:	ASL 2.0
URL: http://github.com/qmx/limonite
Source0:	https://github.com/qmx/limonite/archive/limonite-%{version}-1.tar.gz#/%{name}-%{version}.tar.gz

BuildRequires:	rust-binary

%description


%prep
%setup -qn %{name}-%{version}


%build
cargo build

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}%{_bindir}/
cp -p target/debug/limonite %{buildroot}%{_bindir}/

%files
/usr/bin/limonite
%changelog
* Thu Jan 07 2016 Douglas Campos <qmx@qmx.me> 0.0.4-2
- bump up version on Cargo (qmx@qmx.me)
- fix license on the spec file (qmx@qmx.me)

* Thu Jan 07 2016 Douglas Campos <qmx@qmx.me> 0.0.4-1
- still trying to fix the rpm build (qmx@qmx.me)

* Thu Jan 07 2016 Douglas Campos <qmx@qmx.me> 0.0.3-1
- fixing bogus url, again (qmx@qmx.me)

* Thu Jan 07 2016 Douglas Campos <qmx@qmx.me> 0.0.2-1
- fix url in spec (qmx@qmx.me)

* Thu Jan 07 2016 Douglas Campos <qmx@qmx.me> 0.0.1-1
- new package built with tito


