const std = @import("std");

const RoundData = struct {};

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        printUsageAndExit(args[0]);
    }

    var input_file_path: []const u8 = args[1];

    std.debug.print("Pt1. \n", .{});
    {
        std.debug.print("    Reading file for input data\n", .{});
        var round_data = try getRoundDataFromFilePart1(input_file_path, allocator);

        std.debug.print("    WARNING!!!!! TEMPORARILY ADDING ITEMS TO LIST FOR TEST PURPOSES\n", .{});
        try round_data.append(.{});
        try round_data.append(.{});
        try round_data.append(.{});

        std.debug.print("    Processing {d} rounds\n", .{round_data.items.len});
        var total_score = sumRoundScore(round_data.items);

        std.debug.print("    Total score! {d}\n", .{total_score});
    }
    std.debug.print("\n", .{});
}

pub fn printUsageAndExit(prog_name: []const u8) noreturn {
    std.debug.print("USAGE: {s} [input_file]\n", .{prog_name});
    std.process.exit(0);
}

pub fn getRoundDataFromFilePart1(input_file_path: []const u8, alloc: std.mem.Allocator) !std.ArrayList(RoundData) {
    var rounds = std.ArrayList(RoundData).init(alloc);
    _ = input_file_path;
    return rounds;
}

pub fn sumRoundScore(rounds: []const RoundData) u32 {
    var sum: u32 = 0;
    for (rounds) |round| {
        _ = round;
    }
    return sum;
}

// FIXME: old stub main
// pub fn main() !void {
//     // Prints to stderr (it's a shortcut based on `std.io.getStdErr()`)
//     std.debug.print("All your {s} are belong to us.\n", .{"codebase"});
//
//     // stdout is for the actual output of your application, for example if you
//     // are implementing gzip, then only the compressed bytes should be sent to
//     // stdout, not any debugging messages.
//     const stdout_file = std.io.getStdOut().writer();
//     var bw = std.io.bufferedWriter(stdout_file);
//     const stdout = bw.writer();
//
//     try stdout.print("Run `zig build test` to run the tests.\n", .{});
//
//     try bw.flush(); // don't forget to flush!
// }

// FIXME: stale test from the stub file creation
// test "simple test" {
//     var list = std.ArrayList(i32).init(std.testing.allocator);
//     defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
//     try list.append(42);
//     try std.testing.expectEqual(@as(i32, 42), list.pop());
// }
