import { api } from '$lib/api/client';

// Types matching the Rust domain model
export interface Course {
	id: string;
	name: string;
	provider: string | null;
	url: string | null;
	status: 'active' | 'paused' | 'completed';
	started_at: string | null;
	completed_at: string | null;
	progress_minutes: number;
	progress_notes: string | null;
	created_at: string;
	updated_at: string;
}

export interface CreateCourseInput {
	name: string;
	provider?: string;
	url?: string;
}

// Class-based store using $state rune
class CourseStore {
	items = $state<Course[]>([]);
	loading = $state(false);
	error = $state<string | null>(null);

	// Derived: the currently active course
	get activeCourse(): Course | undefined {
		return this.items.find((c) => c.status === 'active');
	}

	async load() {
		this.loading = true;
		this.error = null;
		try {
			this.items = (await api.listCourses()) as Course[];
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to load courses';
		} finally {
			this.loading = false;
		}
	}

	async create(input: CreateCourseInput) {
		this.error = null;
		try {
			const course = (await api.createCourse(input)) as Course;
			this.items = [course, ...this.items];
			return course;
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to create course';
			throw e;
		}
	}

	async activate(id: string) {
		this.error = null;
		try {
			await api.activateCourse(id);
			// Reload all courses to reflect status changes
			// (activating one course may deactivate others)
			await this.load();
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to activate course';
			throw e;
		}
	}

	async complete(id: string) {
		this.error = null;
		try {
			await api.completeCourse(id);
			// Reload all courses to reflect status changes
			await this.load();
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to complete course';
			throw e;
		}
	}
}

export const courseStore = new CourseStore();
