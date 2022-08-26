#include "mini_uart.h"

void kernel_main(void) {
    uart_init();
    uart_send_string("Hello world!\r\n");

    while (1) { /* infinite loop */
        uart_send(uart_recv()); /* send input back to output */
    }
}