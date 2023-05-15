#!/usr/bin/env python3

import constants

def main():
    content = bytearray()
    content.extend(constants.FILE_MAGIC)
    content.extend(constants.NULL)
    content.extend(constants.UNKNOWN_BYTES)

    f = open("ValidUnknownBytes.fbx", "wb")
    f.write(content)

if __name__ == "__main__":
    main()
