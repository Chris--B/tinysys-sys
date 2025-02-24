/**
 * @file keyringbuffer.c
 * 
 * @brief Keyring buffer implementation
 * 
 * @details Simple ringbuffer
 * 
 * Adapted from DXUT locklesspipe (c) Microsoft
 */

#include "keyringbuffer.h"
#include "task.h"

const static uint32_t cbBufferSizeLog2 = 10; // 1024 bytes
const static uint8_t c_cbBufferSizeLog2 = cbBufferSizeLog2 < 31 ? cbBufferSizeLog2 : 31;
const static uint32_t c_cbBufferSize = ( 1 << c_cbBufferSizeLog2 );
const static uint32_t c_sizeMask = c_cbBufferSize - 1;

// Ring buffer at _task_get_shared_memory()
#define BUFFER_BASE (DEVICE_MAIL + sizeof(struct STaskContext)*MAX_HARTS)
volatile uint32_t *m_readOffset  = (volatile uint32_t *)(BUFFER_BASE + 1024);
volatile uint32_t *m_writeOffset = (volatile uint32_t *)(BUFFER_BASE + 1028);

/**
 * @brief Reset the ring buffer to empty state
 */
void __attribute__ ((noinline)) KeyRingBufferReset()
{
	*m_readOffset = 0;
    *m_writeOffset = 0;
}

/**
 * @brief Read from the ring buffer
 * 
 * Read from the ring buffer into a destination buffer of cbDest bytes.
 * If there are less than cbDest bytes available, no bytes are read.
 * 
 * @param pvDest Destination buffer
 * @param cbDest Number of bytes to read
 * @return Number of bytes read
 */
uint32_t __attribute__ ((noinline)) KeyRingBufferRead(void* pvDest, const uint32_t cbDest)
{
    uint8_t *ringbuffer = (uint8_t *)BUFFER_BASE;

    uint32_t readOffset = *m_readOffset;
    const uint32_t writeOffset = *m_writeOffset;

    const uint32_t cbAvailable = writeOffset - readOffset;
    if( cbDest > cbAvailable )
        return 0;

    //EReadWriteBarrier(0);
    asm volatile ("" : : : "memory"); // Stop compiler reordering

    uint8_t* pbDest = (uint8_t *)pvDest;
    const uint32_t actualReadOffset = readOffset & c_sizeMask;
    uint32_t bytesLeft = cbDest;

    const uint32_t cbTailBytes = bytesLeft < c_cbBufferSize - actualReadOffset ? bytesLeft : c_cbBufferSize - actualReadOffset;
    __builtin_memcpy( pbDest, ringbuffer + actualReadOffset, cbTailBytes );
    bytesLeft -= cbTailBytes;

    //EAssert(bytesLeft == 0, "Item not an exact multiple of ring buffer, this will cause multiple memcpy() calls during Read()");

    readOffset += cbDest;
    *m_readOffset = readOffset;

    return 1;
}

/**
 * @brief Write to the ring buffer
 * 
 * Write to the ring buffer from a source buffer of cbSrc bytes.
 * If the ring buffer is full, no bytes are written.
 * 
 * @param pvSrc Source buffer
 * @param cbSrc Number of bytes to write
 * @return Number of bytes written
 */
uint32_t __attribute__ ((noinline)) KeyRingBufferWrite( const void* pvSrc, const uint32_t cbSrc)
{
    uint8_t *ringbuffer = (uint8_t *)BUFFER_BASE;

    const uint32_t readOffset = *m_readOffset;
    uint32_t writeOffset = *m_writeOffset;

    const uint32_t cbAvailable = c_cbBufferSize - ( writeOffset - readOffset );
    if( cbSrc > cbAvailable )
        return 0;

    const uint8_t* pbSrc = ( const uint8_t* )pvSrc;
    const uint32_t actualWriteOffset = writeOffset & c_sizeMask;
    uint32_t bytesLeft = cbSrc;

    const uint32_t cbTailBytes = bytesLeft < c_cbBufferSize - actualWriteOffset ? bytesLeft : c_cbBufferSize - actualWriteOffset;
    __builtin_memcpy(ringbuffer + actualWriteOffset, pbSrc, cbTailBytes);
    bytesLeft -= cbTailBytes;

    //EAssert(bytesLeft == 0, "Item not an exact multiple of ring buffer, this will cause multiple memcpy() calls during Write()");

    //EReadWriteBarrier(0);
    asm volatile ("" : : : "memory"); // Stop compiler reordering

    writeOffset += cbSrc;
    *m_writeOffset = writeOffset;

    return 1;
}
