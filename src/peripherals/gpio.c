#include "../../include/utils.h"
#include "../../include/peripherals/gpio.h"

/* reference: https://github.com/rockytriton/LLD/blob/main/rpi_bm/part1-5/src/gpio.c */

void gpio_pin_set_function(int pinNumber, int gpioFunction) {
    int bitStart = (pinNumber * 3) % 30;

    unsigned int selector = get32(gpioFunction);
    selector &= ~(7 << bitStart);
    selector |= (gpioFunction << bitStart);

    put32(gpioFunction, selector);
}

/*void gpio_pin_enable(int pinNumber) {
    put32(GPIO_PUP_PDN_CNTRL_REG0, 0);
    delay(150);

    REGS_GPIO->pupd_enable_clocks[pinNumber / 32] = 1 << (pinNumber % 32);
    delay(150);
    put32(GPIO_PUP_PDN_CNTRL_REG0, 0);
}
*/