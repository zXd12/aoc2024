// --- Day 5: Print Queue ---

// I'm using some assumptions on the shape of the data:
// page numbers are 2 digits
// the page count in a update is always odd
// there is always *exactly* one valid ordering for incorrectly-ordered updates

use crate::day_solver::{test_day, DaySolver};

#[inline]
fn parse_page_number(bytes: &[u8], i: usize) -> u8 {
    (bytes[i] - b'0') * 10 + bytes[i+1] - b'0'
}

fn part1(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut i = 0;
    let ordering_list = construct_ordering_list(bytes, &mut i);
    i += 1;
    let mut result = 0;
    'line: while i < bytes.len() {
        let mut disalowed_pages: u128 = 0;
        let mut page_count: usize = 2;
        disalowed_pages |= ordering_list[parse_page_number(bytes, i) as usize - 10];
        i += 3;
        while bytes[i-1] != b'\n' {
            let number = parse_page_number(bytes, i);
            if disalowed_pages & (1 << number) != 0 {
                while bytes[i-1] != b'\n' {
                    i += 3;
                }
                continue 'line;
            }
            disalowed_pages |= ordering_list[number as usize - 10];
            i += 3;
            page_count += 1;
        }
        // the number of pages is always odd
        let middle_page_index = (page_count.div_euclid(2))*3;
        result += parse_page_number(bytes, i - middle_page_index) as u32;
    }
    result.to_string()
}

fn construct_ordering_list(bytes: &[u8], i: &mut usize) -> [u128; 90] {
    let mut ordering_list: [u128; 90] = [0; 90];
    let mut next_char = bytes[*i];
    while next_char != b'\n' {
        // page numbers are Always length 2 in the input
        let number1 = parse_page_number(bytes, *i);
        let number2 = parse_page_number(bytes, *i + 3);
        ordering_list[number2 as usize - 10] |= 1 << number1;
        *i += 6;
        next_char = bytes[*i];
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
        loop {
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
97,13,75,29,47
";

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