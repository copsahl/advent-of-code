// Day one (1)- Find two entries that add to 2020 and then multiply them

fn solve_one(er: &Vec<i32>) -> i32 {
    /* Solution:
        Instead of comparing each number to each number like I had initially wanted to cause I'm lazy...
        I instead subtract the current number in the array from 2020 and then look for that result in the list,
        if it is, then that is our match and we can multiply them and return.
    */
    
    let mut num_to_find: i32;
    for i in er.iter() {
        num_to_find = 2020 - i;
        if er.iter().any(|&x| x==num_to_find) {
            return i * num_to_find;
        }
    }
    return 0;
}

// Day one (2)- Find 3 entries that add to 2020 and then multiply them
fn solve_two(er: &Vec<i32>) -> i32 {
    /*
        Solution:
            Same as the other, but nested xD
    */

    let mut first_num_to_find: i32;
    let mut sec_num_to_find: i32;

    let mut found_one: i32 = 0;

    for i in er.iter() {
        first_num_to_find = 2020 - i;
        for x in er.iter() {
            sec_num_to_find = first_num_to_find - x;
            if er.iter().any(|&y| y == sec_num_to_find) {
                found_one = i * x * sec_num_to_find;
            }
        }
        
    }
    return found_one;

}

fn main() {

    // Yes I copied and pasted the data in here because heck parsing a huge string and plucking out ints
    let expense_report: Vec<i32> = vec![1440,1511,1731,1400,1542,1571,1768,1730,1959,1342,1744,872,1237,1846,1597,1583,1711,1499,1679,1895,1875,1928,1728,1673,481,1934,673,1704,1916,1958,1821,1649,1640,1802,1732,121,1924,1438,1748,1046,1905,1566,1152,1964,1518,1603,1414,1785,1993,1594,1761,1455,1738,1699,1507,1483,1450,1653,1644,19,1340,1227,1353,2009,1188,1228,1898,1941,1515,1766,1351,1980,1378,1702,1620,1729,1279,1384,1894,1770,1853,1161,1970,1986,1669,1938,1602,1190,1822,425,1750,1632,1613,1805,1718,1990,1762,1242,1485,1598,1893,1995,1823,1786,1506,1464,1467,1639,1674,1903,1961,1478,1847,1760,1997,2010,899,2000,1488,1243,1891,1504,1693,1176,1391,1563,692,1497,1428,1745,1368,1723,1989,1930,1171,1840,1372,1987,1952,1842,1967,1759,1929,1945,1919,1333,1692,1811,1221,1520,1920,1093,1618,1795,1686,1369,1820,1857,1356,1562,2004,1519,1628,1831,1687,1792,1948,927,1789,1546,1338,1614,1472,1494,1979,1936,1577,1147,1446,1683,1375,856,1787,1517,1724,1334,1642,1496,1668,1725,1800,1708,1814,1585,1827,1801,1208,1839,1596,1925];

    // Variable shadowing is dope fam
    let result: i32 = solve_one(&expense_report);
    println!("Result 1: {}", result);

    let result: i32 = solve_two(&expense_report);
    println!("Result 2: {}", result);

}
