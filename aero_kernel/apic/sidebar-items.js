initSidebarItems({"constant":[["APIC_SPURIOUS_VECTOR",""],["XAPIC_EOI","EOI register. Write-only."],["XAPIC_ESR","Error Status Register (ESR). Read/write."],["XAPIC_ICR0","Interrupt Command Register (ICR). Read/write."],["XAPIC_ICR1","Interrupt Command Register (ICR). Read/write."],["XAPIC_LVT_ERROR","LVT Error register. Read/write."],["XAPIC_SVR","Spurious Interrupt Vector Register (SVR). Read/write."],["XAPIC_TPR","Task Priority Register (TPR). Read/write. Bits 31:8 are reserved."]],"enum":[["ApicType",""]],"fn":[["ap_ready","Returns true if the AP is ready."],["get_bsp_id","Get the local BSP’s id."],["get_cpu_count","Return the number of active CPUs."],["get_local_apic","Get a mutable reference to the local apic."],["init","Initialize the local apic."],["io_apic_from_redirect","Return the index of the I/O APIC that handles this redirect."],["io_apic_get_max_redirect","Get the maximum number of redirects this I/O APIC can handle."],["io_apic_read","Read from the `io_apic_id` I/O APIC as described by the MADT."],["io_apic_set_redirect",""],["io_apic_setup_legacy_irq",""],["io_apic_write","Write from the `io_apic_id` I/O APIC as described by the MADT."],["is_bsp_ready",""],["mark_ap_ready",""],["mark_bsp_ready",""]],"static":[["AP_READY",""],["BSP_APIC_ID",""],["BSP_READY",""],["CPU_COUNT","The count of all the active CPUs."],["LOCAL_APIC",""]],"struct":[["IoApicHeader",""],["LocalApic",""]]});