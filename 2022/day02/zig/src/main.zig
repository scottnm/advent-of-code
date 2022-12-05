const std = @import("std");

const RpsChoice = enum { rock, paper, scissors };
const RpsResult = enum { lose, draw, win };
const RoundData = struct { opponent_choice: RpsChoice, player_choice: RpsChoice };
const ReadInputError = error{InvalidLineFormat};

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
        defer round_data.deinit();

        std.debug.print("    Processing {d} rounds\n", .{round_data.items.len});
        var total_score = sumRoundScore(round_data.items);

        std.debug.print("    Total score! {d}\n", .{total_score});
    }
    std.debug.print("\n", .{});

    std.debug.print("Pt2. \n", .{});
    {
        std.debug.print("    Reading file for input data\n", .{});
        var round_data = try getRoundDataFromFilePart2(input_file_path, allocator);
        defer round_data.deinit();

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
    const cwd = std.fs.cwd();

    const input_file = try cwd.openFile(input_file_path, .{ .mode = std.fs.File.OpenMode.read_only });
    defer input_file.close();

    const input_file_reader = input_file.reader();
    var read_buffer: [1024]u8 = undefined;

    var rounds = std.ArrayList(RoundData).init(alloc);
    errdefer rounds.deinit();

    while (try input_file_reader.readUntilDelimiterOrEof(read_buffer[0..], '\n')) |line| {
        if (line.len != 3) {
            return ReadInputError.InvalidLineFormat;
        }

        if (line[1] != ' ') {
            return ReadInputError.InvalidLineFormat;
        }

        var opponent_choice_char = line[0];
        var player_choice_char = line[2];

        var opponent_choice = switch (opponent_choice_char) {
            'A' => RpsChoice.rock,
            'B' => RpsChoice.paper,
            'C' => RpsChoice.scissors,
            else => {
                std.debug.assert(false);
                return ReadInputError.InvalidLineFormat;
            },
        };

        var player_choice = switch (player_choice_char) {
            'X' => RpsChoice.rock,
            'Y' => RpsChoice.paper,
            'Z' => RpsChoice.scissors,
            else => {
                std.debug.assert(false);
                return ReadInputError.InvalidLineFormat;
            },
        };

        try rounds.append(.{ .player_choice = player_choice, .opponent_choice = opponent_choice });
    }

    return rounds;
}

pub fn getRoundDataFromFilePart2(input_file_path: []const u8, alloc: std.mem.Allocator) !std.ArrayList(RoundData) {
    const cwd = std.fs.cwd();

    const input_file = try cwd.openFile(input_file_path, .{ .mode = std.fs.File.OpenMode.read_only });
    defer input_file.close();

    const input_file_reader = input_file.reader();
    var read_buffer: [1024]u8 = undefined;

    var rounds = std.ArrayList(RoundData).init(alloc);
    errdefer rounds.deinit();

    while (try input_file_reader.readUntilDelimiterOrEof(read_buffer[0..], '\n')) |line| {
        if (line.len != 3) {
            return ReadInputError.InvalidLineFormat;
        }

        if (line[1] != ' ') {
            return ReadInputError.InvalidLineFormat;
        }

        var opponent_choice_char = line[0];
        var required_result_char = line[2];

        var opponent_choice = switch (opponent_choice_char) {
            'A' => RpsChoice.rock,
            'B' => RpsChoice.paper,
            'C' => RpsChoice.scissors,
            else => {
                std.debug.assert(false);
                return ReadInputError.InvalidLineFormat;
            },
        };

        var required_result = switch (required_result_char) {
            'X' => RpsResult.lose,
            'Y' => RpsResult.draw,
            'Z' => RpsResult.win,
            else => {
                std.debug.assert(false);
                return ReadInputError.InvalidLineFormat;
            },
        };

        var player_choice = switch (required_result) {
            RpsResult.draw => opponent_choice,
            RpsResult.lose => switch (opponent_choice) {
                RpsChoice.rock => RpsChoice.scissors,
                RpsChoice.paper => RpsChoice.rock,
                RpsChoice.scissors => RpsChoice.paper,
            },
            RpsResult.win => switch (opponent_choice) {
                RpsChoice.rock => RpsChoice.paper,
                RpsChoice.paper => RpsChoice.scissors,
                RpsChoice.scissors => RpsChoice.rock,
            },
        };

        try rounds.append(.{ .player_choice = player_choice, .opponent_choice = opponent_choice });
    }

    return rounds;
}

pub fn sumRoundScore(rounds: []const RoundData) u32 {
    var sum: u32 = 0;
    for (rounds) |round| {
        sum += calculateRoundScore(round);
    }
    return sum;
}

pub fn calculateRoundScore(round: RoundData) u32 {
    const roundResult = calculateRoundResult(round);
    const resultScore = calculateResultScore(roundResult);
    const playerChoiceScore = calculateChoiceScore(round.player_choice);
    return resultScore + playerChoiceScore;
}

pub fn calculateRoundResult(round: RoundData) RpsResult {
    if (round.opponent_choice == round.player_choice) {
        return RpsResult.draw;
    } else {
        return switch (round.player_choice) {
            RpsChoice.rock => if (round.opponent_choice == RpsChoice.scissors) RpsResult.win else RpsResult.lose,
            RpsChoice.paper => if (round.opponent_choice == RpsChoice.rock) RpsResult.win else RpsResult.lose,
            RpsChoice.scissors => if (round.opponent_choice == RpsChoice.paper) RpsResult.win else RpsResult.lose,
        };
    }
}

pub fn calculateChoiceScore(player_choice: RpsChoice) u32 {
    return switch (player_choice) {
        RpsChoice.rock => 1,
        RpsChoice.paper => 2,
        RpsChoice.scissors => 3,
    };
}

pub fn calculateResultScore(round_result: RpsResult) u32 {
    return switch (round_result) {
        RpsResult.lose => 0,
        RpsResult.draw => 3,
        RpsResult.win => 6,
    };
}

// FIXME: stale test from the stub file creation
test "simple test round score" {
    var round_data = std.ArrayList(RoundData).init(std.testing.allocator);
    defer round_data.deinit(); // try commenting this out and see if zig detects the memory leak!

    try round_data.append(.{ .opponent_choice = RpsChoice.rock, .player_choice = RpsChoice.paper });
    try round_data.append(.{ .opponent_choice = RpsChoice.paper, .player_choice = RpsChoice.rock });
    try round_data.append(.{ .opponent_choice = RpsChoice.scissors, .player_choice = RpsChoice.scissors });
    var round_score = sumRoundScore(round_data.items);

    try std.testing.expectEqual(round_score, 15);
}
