//
//  AppDelegate.swift
//  RustDataPassing
//
//  Created by John Gallagher on 5/7/15.
//  Copyright (c) 2015 Big Nerd Ranch. All rights reserved.
//

import UIKit

func exercisePrimitives() {
    let a: Int32 = return_int32()
    let b: UInt16 = triple_a_uint16(10)
    let c: Float = return_float()
    let d: Double = average_two_doubles(10, 20)
    let e: Int = sum_sizes(20, 30)
    print("primitives: \(a) \(b) \(c) \(d) \(e)")
}

func exerciseStrings() {
    let swiftString = "Hello from Swift"
    print("Calling c_string_to_rust...")
    c_string_to_rust(swiftString)

    print("Calling utf8_bytes_to_rust...")
    let swiftData = swiftString.dataUsingEncoding(NSUTF8StringEncoding, allowLossyConversion: false)!
    utf8_bytes_to_rust(UnsafePointer<UInt8>(swiftData.bytes), swiftData.length)

    let rustString = get_string_from_rust()

    if let stringFromRust = rustString.asString() {
        print("got a string from Rust: \(stringFromRust)")
    } else {
        print("Could not parse Rust string as UTF-8")
    }
}

@UIApplicationMain
class AppDelegate: UIResponder, UIApplicationDelegate {

    var window: UIWindow?

    func application(application: UIApplication, didFinishLaunchingWithOptions launchOptions: [NSObject: AnyObject]?) -> Bool {

        exercisePrimitives()
        exerciseStrings()

        return true
    }

}

