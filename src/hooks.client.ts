// import type { Handle} from '@sveltejs/kit';

// let ignorePath: Array<string> = ["/auth/login"]

// export const handle: Handle = async ({ event, resolve }) => {
// 	console.log("_----------------------------------------------------------------")
// 	console.log(event)
// 	console.log(event)
// 	console.log("_----------------------------------------------------------------")
// 	const accessToken = event.locals.accessToken;
// 	const refreshToken = event.locals.refreshToken;
// 	const request = event.request;

// 	// Set Authorization header 
// 	const paths: Array<string> = ignorePath.map( (path: string) => import.meta.env.VITE_BASE_URL + path)
// 	if(!request.headers.get('Authorization') && paths.includes(request.url)){
// 		event.setHeaders({"Authorization": "Bearer " + accessToken})
// 	}

// 	let response = await resolve(event);

// 	if(request.url === import.meta.env.BASE_URL + "/auth/login") {
// 		if(response.status === 200){
// 			const {accessToken, refreshToken} = await request.json();
// 			event.locals.accessToken = accessToken;
// 			event.locals.refreshToken = refreshToken;
// 		}else if(response.status === 401){

// 			event.setHeaders({"Authorization": "Bearer " + refreshToken})
// 			response = await resolve(event)

// 		}else{
// 			event.locals.accessToken = null;
// 			event.locals.refreshToken = null;
// 		}
// 	}
// 	return response
// };

