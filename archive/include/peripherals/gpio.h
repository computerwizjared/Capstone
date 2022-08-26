#ifndef	_P_GPIO_H
#define	_P_GPIO_H

#include "peripherals/base.h"

/* reference https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf */
#define GPIO_BASE               (PHYSICAL_BASE + 0x00200000)    /* The start of the the GPIO register addresses */

#define GPFSEL0                 (GPIO_BASE + 0x00)              /* GPIO Function Select 0 */
#define GPFSEL1                 (GPIO_BASE + 0x04)              /* GPIO Function Select 1 */
#define GPFSEL2                 (GPIO_BASE + 0x08)              /* GPIO Function Select 2 */
#define GPFSEL3                 (GPIO_BASE + 0x0C)              /* GPIO Function Select 3 */
#define GPFSEL4                 (GPIO_BASE + 0x10)              /* GPIO Function Select 4 */
#define GPFSEL5                 (GPIO_BASE + 0x14)              /* GPIO Function Select 5 */
#define GPSET0                  (GPIO_BASE + 0x1C)              /* GPIO Pin Output Set 0 */
#define GPSET1                  (GPIO_BASE + 0x20)              /* GPIO Pin Output Set 1 */
#define GPCLR0                  (GPIO_BASE + 0x28)              /* GPIO Pin Output Clear 0 */
#define GPCLR1                  (GPIO_BASE + 0x2c)              /* GPIO Pin Level 1 */
#define GPLEV0                  (GPIO_BASE + 0x34)              /* GPIO Pin Level 0 */
#define GPLEV1                  (GPIO_BASE + 0x38)              /* GPIO Pin Level 1 */
#define GPEDS0                  (GPIO_BASE + 0x40)              /* GPIO Pin Event Detect Status 0 */
#define GPEDS1                  (GPIO_BASE + 0x44)              /* GPIO Pin Event Detect Status 1 */
#define GPREN0                  (GPIO_BASE + 0x4C)              /* GPIO Pin Rising Edge Detect Enable 0 */
#define GPREN1                  (GPIO_BASE + 0x50)              /* GPIO Pin Rising Edge Detect Enable 1 */
#define GPFEN0                  (GPIO_BASE + 0x58)              /* GPIO Pin Falling Edge Detect Enable 0 */
#define GPFEN1                  (GPIO_BASE + 0x5C)              /* GPIO Pin Falling Edge Detect Enable 1 */
#define GPHEN0                  (GPIO_BASE + 0x64)              /* GPIO Pin High Detect Enable 0 */
#define GPHEN1                  (GPIO_BASE + 0x68)              /* GPIO Pin High Detect Enable 1 */
#define GPLEN0                  (GPIO_BASE + 0x70)              /* GPIO Pin Low Detect Enable 0 */
#define GPLEN1                  (GPIO_BASE + 0x74)              /* GPIO Pin Low Detect Enable 1 */
#define GPAREN0                 (GPIO_BASE + 0x7C)              /* GPIO Pin Async. Rising Edge Detect 0 */
#define GPAREN1                 (GPIO_BASE + 0x80)              /* GPIO Pin Async. Rising Edge Detect 1 */
#define GPAFEN0                 (GPIO_BASE + 0x88)              /* GPIO Pin Async. Falling Edge Detect 0 */
#define GPAFEN1                 (GPIO_BASE + 0x8C)              /* GPIO Pin Async. Falling Edge Detect 1 */
#define GPIO_PUP_PDN_CNTRL_REG0 (GPIO_BASE + 0xE4)              /* GPIO Pull-up / Pull-down Enable 0 */
#define GPIO_PUP_PDN_CNTRL_REG1 (GPIO_BASE + 0x94)              /* GPIO Pull-up / Pull-down Enable 1 */
#define GPIO_PUP_PDN_CNTRL_REG2 (GPIO_BASE + 0xEC)              /* GPIO Pull-up / Pull-down Enable 2 */
#define GPIO_PUP_PDN_CNTRL_REG3 (GPIO_BASE + 0xF0)              /* GPIO Pull-up / Pull-down Enable 3 */

/* GPIO Functions */
#define GF_INPUT 0x000
#define GF_OUTPUT 0x001
#define GF_ALT0 0x100
#define GF_ALT1 0x101
#define GF_ALT2 0x110
#define GF_ALT3 0x111
#define GF_ALT4 0x011
#define GF_ALT5 0x010

void gpio_pin_set_function(int pinNumber, int gpioFunction);
void gpio_pin_enable(int pinNumber);

#endif