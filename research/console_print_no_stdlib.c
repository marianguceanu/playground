int _start() {
        const char hello[] = "Hello, world!\n";

        asm volatile(
            "mov $1,  %%rax\n\t" // 1 is the syscall number for write
            "mov $1,  %%rdi\n\t" // 1 is also the stdout, because fd = 1
            "lea 0,   %%rsi\n\t" // address of message, 0 since it's the first
                                 // line
            "mov $14, %%rdx\n\t" // length
            "syscall\n\t"
            :
            : "m"(hello)
            : "%rax", "%rdi", "rsi", "rdx");

        // Now we must execute the exit syscall
        asm volatile("mov $60, %%rax\n\t" // 60 is SYS_exit syscall code
                     "mov $0,  %%rdi\n\t" // 0 is the exit code
                     "syscall\n\t" ::
                         : "%rax", "%rdi");
        return 0;
}

// gcc -static -nostdlib main.c -o main

