#[derive(Copy, Clone, Debug)]
struct Point {
    x: f32,
    y: f32,
}

fn line_point_distance(l1: Point, l2: Point, p: Point) -> f32 {
    let numerator = ((l2.y - l1.y) * p.x - (l2.x - l1.x) * p.y + l2.x * l1.y - l2.y * l1.x).abs();
    let denominator = ((l2.y - l1.y) * (l2.y - l1.y) + (l2.x - l1.x) * (l2.x - l1.x)).sqrt();
    let distance = numerator / denominator;
    return distance;
}

fn print_vec(v: &Vec<Point>) {
    for p in v {
        println!("({}, {})", p.x, p.y);
    }
}

fn douglas_peucker(points: Vec<Point>, eps: f32) -> Vec<Point> {
    let mut result_list: Vec<Point> = vec![];
    let mut dmax = 0.0;
    let mut index: usize = 0;
    let mut d = 0.0;
    for i in 1..points.len() - 1 {
        d = line_point_distance(points[0], points[points.len() - 1], points[i]);
        if d > dmax {
            dmax = d;
            index = i;
        }
    }

    if dmax > eps {
        let rec_results1 = douglas_peucker(points[0 .. index + 1].to_vec(), eps);
        let rec_results2 = douglas_peucker(points[index .. (points.len())].to_vec(), eps);
        result_list = rec_results1[0 .. rec_results1.len() - 1].to_vec();
        result_list.extend(rec_results2.iter().cloned());
    }
    else {
        result_list.push(points[0]);
        result_list.push(points[points.len() - 1]);
    }

    return result_list;
}

fn main() {
    let l1 = Point {x:0.0, y:0.0};
    let l2 = Point {x:5.0, y:0.0};
    let l3 = Point {x:10.0, y:5.0};
    let l4 = Point {x:20.0, y:1.0};
    let l5 = Point {x:25.0, y:0.0};
    let points: Vec<Point> = vec![l1, l2, l3, l4, l5];
    let result = douglas_peucker(points, 1.0);
    println!("RESULTS");
    print_vec(&result);
}
