#!/usr/bin/env python3

import constants

def main():
    content = bytearray()
    content.extend(constants.FILE_MAGIC)
    content.extend(constants.NULL)
    content.extend(b'\x12\x34')
    
    f = open("MalformedUnknownByte.fbx", "wb")
    f.write(content)

if __name__ == "__main__":
    main()
