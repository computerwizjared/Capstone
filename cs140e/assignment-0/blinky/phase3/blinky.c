#define GPIO_BASE (0x3F000000 + 0x200000)

volatile unsigned *GPIO_FSEL1 = (volatile unsigned *)(GPIO_BASE + 0x04);
volatile unsigned *GPIO_SET0  = (volatile unsigned *)(GPIO_BASE + 0x1C);
volatile unsigned *GPIO_CLR0  = (volatile unsigned *)(GPIO_BASE + 0x28);

static void spin_sleep_us(unsigned int us) {
  for (unsigned int i = 0; i < us * 6; i++) {
    asm volatile("nop");
  }
}

static void spin_sleep_ms(unsigned int ms) {
  spin_sleep_us(ms * 1000);
}

int main(void) {
  // 18 is the lower bit for pin 16 (bits 20-18)
  // | is bitwise OR operator
  *GPIO_FSEL1 |= (0b001 << 18); 
  
  // infinite loop
  for (;;) {
    // set pin 16 (turn it on)
    *GPIO_SET0 |= (0b1 << 16);
    spin_sleep_ms(500);
    
    // clear pin 16 (turn it off)
    *GPIO_CLR0 |= (0b1 << 16);
    spin_sleep_ms(500);
  }
}
