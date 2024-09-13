// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			user?:{
				id: number;
				username: string;
				nickname: string;
				email: string;
			},
			accessToken?: string | null;
			refreshToken?: string | null;
		}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
		interface Error {
			message: string;
			errorId: string;
		}


	}
}

export {
	
};


