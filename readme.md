# Description

Merge /proc/<pid>/maps with elf section name and offset infos


# Usage

```
$ elf-maps <pid>
```

example:
```
$ elf-maps $(pidof firefox | cut -d' ' -f1)
7f420a5e2000-7f420a5eb000 r--p                          00000000 00000000 /usr/lib/firefox/libmozsandbox.so
7f420a5e2000-7f420a5eb000 r--p .note.gnu.build-id       00000000 00000270 /usr/lib/firefox/libmozsandbox.so
7f420a5e2000-7f420a5eb000 r--p .dynsym                  00000000 00000298 /usr/lib/firefox/libmozsandbox.so
7f420a5e2000-7f420a5eb000 r--p .gnu.version             00000000 00000c10 /usr/lib/firefox/libmozsandbox.so
7f420a5e2000-7f420a5eb000 r--p .gnu.version_r           00000000 00000cdc /usr/lib/firefox/libmozsandbox.so
7f420a5e2000-7f420a5eb000 r--p .gnu.hash                00000000 00000e10 /usr/lib/firefox/libmozsandbox.so
7f420a5e2000-7f420a5eb000 r--p .hash                    00000000 00000e98 /usr/lib/firefox/libmozsandbox.so
7f420a5e2000-7f420a5eb000 r--p .dynstr                  00000000 000011c8 /usr/lib/firefox/libmozsandbox.so
7f420a5e2000-7f420a5eb000 r--p .rela.dyn                00000000 00001bd8 /usr/lib/firefox/libmozsandbox.so
7f420a5e2000-7f420a5eb000 r--p .rela.plt                00000000 000030f0 /usr/lib/firefox/libmozsandbox.so
7f420a5e2000-7f420a5eb000 r--p .rodata                  00000000 00003150 /usr/lib/firefox/libmozsandbox.so
7f420a5e2000-7f420a5eb000 r--p .eh_frame_hdr            00000000 00004bf4 /usr/lib/firefox/libmozsandbox.so
7f420a5e2000-7f420a5eb000 r--p .eh_frame                00000000 000054e0 /usr/lib/firefox/libmozsandbox.so
7f420a5eb000-7f420a5fa000 r-xp .init                    00008000 000084cc /usr/lib/firefox/libmozsandbox.so
7f420a5eb000-7f420a5fa000 r-xp .fini                    00008000 000084e8 /usr/lib/firefox/libmozsandbox.so
7f420a5eb000-7f420a5fa000 r-xp .text                    00008000 00008500 /usr/lib/firefox/libmozsandbox.so
7f420a5fa000-7f420a5fc000 r--p .plt                     00016000 00016850 /usr/lib/firefox/libmozsandbox.so
7f420a5fa000-7f420a5fc000 r--p .fini_array              00016000 000168a0 /usr/lib/firefox/libmozsandbox.so
7f420a5fa000-7f420a5fc000 r--p .init_array              00016000 000168a8 /usr/lib/firefox/libmozsandbox.so
7f420a5fa000-7f420a5fc000 r--p .data.rel.ro             00016000 000168c0 /usr/lib/firefox/libmozsandbox.so
7f420a5fa000-7f420a5fc000 r--p .dynamic                 00016000 00016e80 /usr/lib/firefox/libmozsandbox.so
7f42088f4000-7f42088f7000 r--p                          00000000 00000000 /usr/lib/libz.so.1.2.11
7f42088f4000-7f42088f7000 r--p .note.gnu.build-id       00000000 00000238 /usr/lib/libz.so.1.2.11
7f42088f4000-7f42088f7000 r--p .gnu.hash                00000000 00000260 /usr/lib/libz.so.1.2.11
7f42088f4000-7f42088f7000 r--p .dynsym                  00000000 00000760 /usr/lib/libz.so.1.2.11
7f42088f4000-7f42088f7000 r--p .dynstr                  00000000 00001288 /usr/lib/libz.so.1.2.11
7f42088f4000-7f42088f7000 r--p .gnu.version             00000000 000017ec /usr/lib/libz.so.1.2.11
7f42088f4000-7f42088f7000 r--p .gnu.version_d           00000000 000018e0 /usr/lib/libz.so.1.2.11
7f42088f4000-7f42088f7000 r--p .gnu.version_r           00000000 00001ac8 /usr/lib/libz.so.1.2.11
7f42088f4000-7f42088f7000 r--p .rela.dyn                00000000 00001b08 /usr/lib/libz.so.1.2.11
7f42088f7000-7f4208905000 r-xp .init                    00003000 00003000 /usr/lib/libz.so.1.2.11
7f42088f7000-7f4208905000 r-xp .text                    00003000 00003020 /usr/lib/libz.so.1.2.11
7f42088f7000-7f4208905000 r-xp .fini                    00003000 00010270 /usr/lib/libz.so.1.2.11
7f4208905000-7f420890b000 r--p .rodata                  00011000 00011000 /usr/lib/libz.so.1.2.11
...
```
