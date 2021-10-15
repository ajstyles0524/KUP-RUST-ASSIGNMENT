///Coordinate enum which used to encapsulate the X_axis as Abscissa and Y_axis as Ordinate
///
/// #variant
///
/// Abscissa:-Abscissa is variant of enum Coordinate and associated with {integer} type
///
/// Ordinate:-Abscissa is variant of enum Coordinate and associated with {integer} type
#[derive(PartialEq, Eq, Debug)]
pub enum Coordinate {
    Abscissa(i32),
    Ordinate(i32),
}
///Position enum which used to describe the Position of Quadrant
///
/// #field
///
/// First:- First is field of enum Position and associated with Coordinate type
///
/// Second:- Second is field of enum Position and associated with Coordinate type
///
/// Third:-  Third is field of enum Position and associated with Coordinate type
///
/// Fourth: Fourth is field of enum Position and associated with Coordinate type
///
/// XAxis: XAxis is field of enum Position and associated with Coordinate type
///
/// YAxis: YAxis is field of enum Position and associated with Coordinate type
///
/// Origin: Origin is field of enum Position and associated with Coordinate type
#[derive(PartialEq, Eq, Debug)]
pub enum Position {
    First(Coordinate, Coordinate),
    Second(Coordinate, Coordinate),
    Third(Coordinate, Coordinate),
    Fourth(Coordinate, Coordinate),
    XAxis(Coordinate, Coordinate),
    YAxis(Coordinate, Coordinate),
    Origin(Coordinate, Coordinate),
}
/// check_coordinate which used check the Quadrant of the given point
///
/// #Arguments
///
///point: A tuple of two integer type value
///
/// #Return
///
/// Returns Result enum which used give the Position lied point and handle Error as well....
pub fn check_coordinate(point: (i32, i32)) -> Result<Position, String> {
    match point {
        (x_axis, y_axis) if x_axis > 0 && y_axis > 0 => Ok(Position::First(
            Coordinate::Abscissa(point.0),
            Coordinate::Ordinate(point.1),
        )),
        (x_axis, y_axis) if x_axis < 0 && y_axis > 0 => Ok(Position::Second(
            Coordinate::Abscissa(point.0),
            Coordinate::Ordinate(point.1),
        )),
        (x_axis, y_axis) if x_axis < 0 && y_axis < 0 => Ok(Position::Third(
            Coordinate::Abscissa(point.0),
            Coordinate::Ordinate(point.1),
        )),
        (x_axis, y_axis) if x_axis > 0 && y_axis < 0 => Ok(Position::Fourth(
            Coordinate::Abscissa(point.0),
            Coordinate::Ordinate(point.1),
        )),
        (x_axis, y_axis) if x_axis == 0 && y_axis != 0 => Ok(Position::YAxis(
            Coordinate::Abscissa(point.0),
            Coordinate::Ordinate(point.1),
        )),
        (x_axis, y_axis) if x_axis != 0 && y_axis == 0 => Ok(Position::XAxis(
            Coordinate::Abscissa(point.0),
            Coordinate::Ordinate(point.1),
        )),
        (x_axis, y_axis) if x_axis == 0 && y_axis == 0 => Ok(Position::Origin(
            Coordinate::Abscissa(point.0),
            Coordinate::Ordinate(point.1),
        )),
        _ => Err("Wrong Ip".to_string()),
    }
}
fn main() {
    println!("Assignment-5(check_coordinate)");
    let point_check = (-2, 3);
    println!("{:?}", check_coordinate(point_check));
}
