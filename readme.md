# Description

Merge /proc/<pid>/maps with elf section name and offset infos


# Usage

```
Usage: elf-maps <pid>
Output format:
start-end permission section_name maps_offset elf_offset section_size filename
```

example:
```
$ elf-maps $(pidof sleep)
7faf65610000-7faf65610000 r--p                          00000000 00000000 00000000 /usr/lib/libc-2.33.so
7faf65610350-7faf656103a0 r--p .note.gnu.property       00000000 00000350 00000050 /usr/lib/libc-2.33.so
7faf656103a0-7faf656103c4 r--p .note.gnu.build-id       00000000 000003a0 00000024 /usr/lib/libc-2.33.so
7faf656103c4-7faf656103e4 r--p .note.ABI-tag            00000000 000003c4 00000020 /usr/lib/libc-2.33.so
7faf656103e8-7faf6561399c r--p .hash                    00000000 000003e8 000035b4 /usr/lib/libc-2.33.so
7faf656139a0-7faf65617710 r--p .gnu.hash                00000000 000039a0 00003d70 /usr/lib/libc-2.33.so
7faf65617710-7faf656259c0 r--p .dynsym                  00000000 00007710 0000e2b0 /usr/lib/libc-2.33.so
7faf656259c0-7faf6562bce2 r--p .dynstr                  00000000 000159c0 00006322 /usr/lib/libc-2.33.so
7faf6562bce2-7faf6562cfc6 r--p .gnu.version             00000000 0001bce2 000012e4 /usr/lib/libc-2.33.so
7faf6562cfc8-7faf6562d480 r--p .gnu.version_d           00000000 0001cfc8 000004b8 /usr/lib/libc-2.33.so
7faf6562d480-7faf6562d4c0 r--p .gnu.version_r           00000000 0001d480 00000040 /usr/lib/libc-2.33.so
7faf6562d4c0-7faf65634f60 r--p .rela.dyn                00000000 0001d4c0 00007aa0 /usr/lib/libc-2.33.so
7faf65634f60-7faf656353e0 r--p .rela.plt                00000000 00024f60 00000480 /usr/lib/libc-2.33.so
7faf6565c000-7faf6565c310 r-xp .plt                     00026000 00026000 00000310 /usr/lib/libc-2.33.so
7faf6565c310-7faf6565c350 r-xp .plt.got                 00026000 00026310 00000040 /usr/lib/libc-2.33.so
7faf6565c350-7faf6565c650 r-xp .plt.sec                 00026000 00026350 00000300 /usr/lib/libc-2.33.so
7faf6565c650-7faf657a616d r-xp .text                    00026000 00026650 00149b1d /usr/lib/libc-2.33.so
7faf657a6170-7faf657a709c r-xp __libc_freeres_fn        00026000 00170170 00000f2c /usr/lib/libc-2.33.so
7faf658f4000-7faf659188b8 r--p .rodata                  00172000 00172000 000248b8 /usr/lib/libc-2.33.so
7faf659188b8-7faf659188b9 r--p .stapsdt.base            00172000 001968b8 00000001 /usr/lib/libc-2.33.so
7faf659188c0-7faf659188de r--p .interp                  00172000 001968c0 0000001e /usr/lib/libc-2.33.so
7faf659188e0-7faf6591ec0c r--p .eh_frame_hdr            00172000 001968e0 0000632c /usr/lib/libc-2.33.so
7faf6591ec10-7faf6593ed20 r--p .eh_frame                00172000 0019cc10 00020110 /usr/lib/libc-2.33.so
7faf6593ed20-7faf6593f12a r--p .gcc_except_table        00172000 001bcd20 0000040a /usr/lib/libc-2.33.so
7faf6598b4e0-7faf6598b4f0 r--p .tdata                   001bd000 001bd4e0 00000010 /usr/lib/libc-2.33.so
7faf6598b4f0-7faf6598b568 r--p .tbss                    001bd000 001bd4f0 00000078 /usr/lib/libc-2.33.so
7faf6598b4f0-7faf6598b500 r--p .init_array              001bd000 001bd4f0 00000010 /usr/lib/libc-2.33.so
7faf6598b500-7faf6598d9c0 r--p .data.rel.ro             001bd000 001bd500 000024c0 /usr/lib/libc-2.33.so
7faf6598d9c0-7faf6598dba0 r--p .dynamic                 001bd000 001bf9c0 000001e0 /usr/lib/libc-2.33.so
7faf6598dba0-7faf6598dff0 r--p .got                     001bd000 001bfba0 00000450 /usr/lib/libc-2.33.so
55c35fc5f000-55c35fc5f000 r--p                          00000000 00000000 00000000 /usr/bin/sleep
55c35fc5f2a8-55c35fc5f2c4 r--p .interp                  00000000 000002a8 0000001c /usr/bin/sleep
55c35fc5f2c4-55c35fc5f2e8 r--p .note.gnu.build-id       00000000 000002c4 00000024 /usr/bin/sleep
55c35fc5f2e8-55c35fc5f308 r--p .note.ABI-tag            00000000 000002e8 00000020 /usr/bin/sleep
55c35fc5f308-55c35fc5f378 r--p .gnu.hash                00000000 00000308 00000070 /usr/bin/sleep
55c35fc5f378-55c35fc5f900 r--p .dynsym                  00000000 00000378 00000588 /usr/bin/sleep
55c35fc5f900-55c35fc5fbb8 r--p .dynstr                  00000000 00000900 000002b8 /usr/bin/sleep
55c35fc5fbb8-55c35fc5fc2e r--p .gnu.version             00000000 00000bb8 00000076 /usr/bin/sleep
55c35fc5fc30-55c35fc5fc90 r--p .gnu.version_r           00000000 00000c30 00000060 /usr/bin/sleep
55c35fc5fc90-55c35fc60368 r--p .rela.dyn                00000000 00000c90 000006d8 /usr/bin/sleep
55c35fc60368-55c35fc60380 r--p .rela.plt                00000000 00001368 00000018 /usr/bin/sleep
55c35fc63000-55c35fc6301b r-xp .init                    00002000 00002000 0000001b /usr/bin/sleep
55c35fc63020-55c35fc63040 r-xp .plt                     00002000 00002020 00000020 /usr/bin/sleep
55c35fc63040-55c35fc66a22 r-xp .text                    00002000 00002040 000039e2 /usr/bin/sleep
...
```
