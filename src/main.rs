use sqlx::mysql::{MySqlPoolOptions};
use sqlx::{query, query_as};
use chrono;
use chrono::DateTime;

#[derive(sqlx::FromRow)]
#[derive(Debug)]
struct Student {
	first_name: String,
	surname: String,
	form: Option<String>,
	student_id: i32 // Primary Key
}

#[derive(sqlx::FromRow)]
#[derive(Debug)]
struct Course {
	course_code: i32, // Primary Key
	course_name: String,
	exam_board: String
}

#[derive(sqlx::FromRow, Debug)]
struct Enrollment {
	start_date: chrono::DateTime<chrono::FixedOffset>,
	student_id: i32, // Foreign composite key
	course_code: i32, // Foreign composite key
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://tallie:8CA8a6c284@localhost/a_levels").await?;

	println!("\u{001b}[1mChecking tables...\u{001b}[0m");
	query!("CREATE TABLE IF NOT EXISTS students (first_name TEXT NOT NULL, surname TEXT NOT NULL, form TEXT, student_id INTEGER NOT NULL AUTO_INCREMENT, PRIMARY KEY (student_id));")
		.execute(&pool).await?;
	query!("CREATE TABLE IF NOT EXISTS courses (course_code INTEGER NOT NULL AUTO_INCREMENT, course_name TEXT NOT NULL, exam_board TEXT NOT NULL, PRIMARY KEY (course_code));")
		.execute(&pool).await?;
	query!("CREATE TABLE IF NOT EXISTS enrolled (student_id INTEGER NOT NULL, course_code INTEGER NOT NULL, date_started DATE, PRIMARY KEY (student_id, course_code), FOREIGN KEY (student_id) REFERENCES students(student_id) ON DELETE CASCADE, FOREIGN KEY (course_code) REFERENCES courses(course_code) ON DELETE CASCADE);")
		.execute(&pool).await?;

	if query_as!(Student, "SELECT * FROM students;")
		.fetch_all(&pool)
		.await?.len() == 0 {
			setup_student_table(&pool).await?;
		}
	if query_as!(Course, "SELECT * FROM courses;")
		.fetch_all(&pool)
		.await?.len() == 0 {
			setup_courses_table(&pool).await?;
		}
    Ok(())
}

async fn setup_student_table(pool: &sqlx::Pool<sqlx::MySql>) -> Result<(), sqlx::Error> {
	println!("\u{001b}[1;38;5;214mInitialising table `students` with default data\u{001b}[0m");
	let students: Vec<Student> = vec![
		Student { first_name: "Tallie".into(), surname: "Tye".into(), form: Some("12MW".into()) , student_id: 0},
		Student { first_name: "Amelia".into(), surname: "Quinlivan".into(), form: Some("12KTA".into()) , student_id: 1},
		Student { first_name: "John".into(), surname: "Doe".into(), form: None , student_id: 2}
	];

	for student in students {
		query_as!(Student,
		    "INSERT INTO students (first_name, surname, form) VALUES (?, ?, ?)",
		    student.first_name,
		    student.surname,
		    student.form
		).execute(pool).await?;
	}
	Ok(())
}

async fn setup_courses_table(pool: &sqlx::Pool<sqlx::MySql>) -> Result<(), sqlx::Error> {
	println!("\u{001b}[1;38;5;214mInitialising table `courses` with default data\u{001b}[0m");
	let courses: Vec<Course> = vec![
		Course { course_code: 0, course_name: "Computer Science".into(), exam_board: "OCR".into() },
		Course { course_code: 1, course_name: "Further Maths".into(), exam_board: "OCR".into() },
		Course { course_code: 2, course_name: "History".into(), exam_board: "Edexcel".into() }
	];
	for course in courses {
		query_as!(Course,
		    "INSERT INTO courses(course_name, exam_board) VALUES (?, ?)",
			course.course_name,
			course.exam_board
		).execute(pool).await?;
	}
	Ok(())
}

async fn setup_enrolled_table(pool: &sqlx::Pool<sqlx::MySql>) -> Result<(), sqlx::Error> {
	println!("\u{001b}[1;38;5;214mInitialising table `enrolled` with default data\u{001b}[0m");
	let enrollments: Vec<Enrollment> = vec![
		Enrollment { student_id: 0, course_code: 1, start_date: DateTime::parse_from_rfc2822("4 September 2022 00:00:00").unwrap() },
		Enrollment { student_id: 0, course_code: 0, start_date: DateTime::parse_from_rfc2822("5 September 2022 00:00:00").unwrap() },
		Enrollment { student_id: 1, course_code: 2, start_date: DateTime::parse_from_rfc2822("4 September 2022 00:00:00").unwrap() }
	];

	Ok(())
}
