int _start() {
        const char hello[] = "Hello, world!\n";

        asm volatile(
            "mov $1,  %%eax\n\t" // 1 is the syscall number for write
            "mov $1,  %%edi\n\t" // 1 is also the stdout, because fd = 1
            "lea 0,   %%esi\n\t" // address of message, 0 since it's the first
                                 // line
            "mov $14, %%edx\n\t" // length
            "syscall\n\t"
            :
            : "m"(hello)
            : "%eax", "%edi", "esi", "edx");

        // Now we must execute the exit syscall
        asm volatile("mov $60, %%eax\n\t" // 60 is SYS_exit syscall code
                     "mov $0,  %%edi\n\t" // 0 is the exit code
                     "syscall\n\t" ::
                         : "%eax", "%edi");
        return 0;
}

// gcc -static -nostdlib main.c -o main
