initSidebarItems({"constant":[["ICW1_ICW4",""],["ICW1_INIT",""],["ICW4_8086",""],["IDT_ENTRIES","Declare an IDT of 256 entries. Although not all entries are used, the rest exists as if any undefined IDT entry is hit, it will cause an \"Unhandled Interrupt\" exception."],["PIC1_COMMAND",""],["PIC1_DATA",""],["PIC2_COMMAND",""],["PIC2_DATA",""],["PIC_EOI",""]],"fn":[["enable_interrupts","Enable interrupts."],["end_pic1",""],["init","Initialize the IDT."],["load_idt","Load the IDT using inline assembly."],["load_pic",""]],"static":[["IDT",""]],"struct":[["IDTDescriptor",""],["IDTEntry",""],["IDTFlags",""]],"type":[["IDTInterruptHandlerFn",""]]});