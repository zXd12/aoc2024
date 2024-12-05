// --- Day 5: Print Queue ---

use crate::day_solver::{test_day, DaySolver};

fn part1(input: &str) -> String {
    let mut lines = input.lines();
    let ordering_list = construct_ordering_list(&mut lines);
    let mut result = 0;
    'line: for line in lines {
        let mut disalowed_pages: u128 = 0;
        let line_bytes = line.as_bytes();
        let page_count = (line_bytes.len()+1)/3;
        for i in 0..page_count {
            let number = (line_bytes[i*3] - b'0') * 10 + line_bytes[i*3+1] - b'0';
            if disalowed_pages & (1 << number) != 0 {
                while bytes[i-1] != b'\n' {
                    i += 3;
                }
                continue 'line;
            }
            disalowed_pages |= ordering_list[number as usize - 10];
        }
        // the number of pages is always odd
        let middle_page_index = (page_count.div_euclid(2))*3;
        result += ((line_bytes[middle_page_index] - b'0') * 10 + line_bytes[middle_page_index+1] - b'0') as u32;
    }
    result.to_string()
}

fn construct_ordering_list(lines: &mut std::str::Lines) -> [u128; 90] {
    let mut ordering_list: [u128; 90] = [0; 90];
    let mut current_line = lines.next().unwrap().as_bytes();
    while current_line.len() != 0 {
        // page numbers are Always length 2 in the input
        let number1 = parse_page_number(bytes, *i);
        let number2 = parse_page_number(bytes, *i + 3);
        ordering_list[number2 as usize - 10] |= 1 << number1;
        current_line = lines.next().unwrap().as_bytes();
    }
    ordering_list
}

fn part2(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut i = 0;
    let ordering_list = construct_ordering_list(bytes, &mut i);
    i += 1;
    let mut result = 0;
    'lines: while i < bytes.len() {
        let mut disalowed_pages: u128 = 0;
        let mut pages = 0;
        let number = parse_page_number(bytes, i);
        pages |= 1 << number;
        disalowed_pages |= ordering_list[number as usize - 10];
        i += 3;
        while bytes[i-1] != b'\n' {
            let number = parse_page_number(bytes, i);
            pages |= 1 << number;
            if disalowed_pages & (1 << number) != 0 {
                while bytes[i-1] != b'\n' {
                    let number = parse_page_number(bytes, i);
                    pages |= 1 << number;
                    i += 3;
                }
                result += reorder_pages(pages, ordering_list);
                continue 'lines;
            }
            disalowed_pages |= ordering_list[number as usize - 10];
            i += 3;
        }
    }
    result.to_string()
}

fn reorder_pages(mut pages: u128, ordering_list: [u128; 90]) -> u32 {
    let mut pages_left_before_middle = pages.count_ones().div_euclid(2);
    loop {
        let mut pages_iter = pages;
        while  pages_iter != 0 {
            let page = pages_iter.trailing_zeros();
            if pages & ordering_list[page as usize - 10] == 0 {
                if pages_left_before_middle <= 0 {
                    return page
                }
                pages_left_before_middle -= 1;
                pages ^= 1 << page;
                break;
            }
            pages_iter &= pages_iter - 1;
        }
    }
}

const SAMPLE_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13
75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

pub const SOLVER: DaySolver = DaySolver::new(
    part1,
    SAMPLE_INPUT,
    "143",
    part2,
    SAMPLE_INPUT,
    "123",
    5,
);

test_day!(SOLVER);