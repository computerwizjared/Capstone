#ifndef	_UTILS_H
#define	_UTILS_H

extern void delay(unsigned long);               /* delay for a specified amount of time */
extern void put32(unsigned long, unsigned int); /* write to a 32-bit register */
extern unsigned int get32(unsigned long);       /* read from a 32-bit register */

#endif