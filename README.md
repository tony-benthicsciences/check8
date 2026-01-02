# Check8 – A library for computing 8-bit checksums

This library provides a number of common 8-bit checksum algorithms, implemented using a common interface defined by the Check8 trait.

It is written primarily as a teaching and learning exercise.

## Notes

- The library includes implementations for CRC-8, CRC-8/ATM, CRC-8/CDMA2000, CRC-8/DARC, CRC-8/ETSI, CRC-8/ROHC, CRC-8/SMBUS, and CRC-8/WCDMA.
- Each algorithm is implemented as a struct that implements the Check8 trait.
- The Check8 trait defines methods for initialising the checksum, adding data to the checksum, and retrieving the final checksum value.
- The library includes unit tests for each algorithm to ensure correctness.

## AI-Generated Code

As an exercise, Check8, Check8Sum, and Check8Xor were hand-crafted. Check8Crc was generated entirely by the Junie AI agent in JetBrains RustRover.
