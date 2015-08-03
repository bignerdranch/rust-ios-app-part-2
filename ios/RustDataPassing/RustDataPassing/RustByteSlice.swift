//
//  RustByteSlice.swift
//  RustDataPassing
//
//  Created by John Gallagher on 7/26/15.
//  Copyright © 2015 Big Nerd Ranch. All rights reserved.
//

import Foundation

extension RustByteSlice {
    func asUnsafeBufferPointer() -> UnsafeBufferPointer<UInt8> {
        return UnsafeBufferPointer(start: bytes, count: len)
    }

    func asString(encoding: NSStringEncoding = NSUTF8StringEncoding) -> String? {
        return String(bytes: asUnsafeBufferPointer(), encoding: encoding)
    }
}
