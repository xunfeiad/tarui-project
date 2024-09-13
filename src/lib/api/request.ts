import { goto } from "$app/navigation";
import { progress } from "../store";

const enum PostType {
  Json,
  UrlEncoded,
  FormData,
}

interface RequestOptions {
  method?: "GET" | "POST" | "PUT" | "PATCH" | "DELETE" | "HEAD";
  headers?: Headers;
  body?: BodyInit;
}

// 定义拦截器的接口
interface Intercepter<T> {
  onFulfilled?: (value: T) => T | Promise<T>;
  onRejected?: (error: T) => T;
}

// 定义拦截器管理类-用于管理多个拦截器，可以通过 `use()`方法相拦截器数组中添加一个拦截器，通过`forEach()`方法对所有的拦截器进行遍历和执行
class InterceptorManager<T> {
  private interceptors: Array<Intercepter<T>>;
  constructor() {
    this.interceptors = [];
  }

  use(intercepor: Intercepter<T>) {
    this.interceptors.push(intercepor);
  }

  forEach(fn: (interceptor: Intercepter<T>) => void) {
    this.interceptors.forEach((interceptor) => {
      if (interceptor) {
        fn(interceptor);
      }
    });
  }
}

// 添加拦截器的 `request` 函数
async function request<T>(
  url: string,
  options: RequestOptions = {}
): Promise<any> {
  const requestInterceptors = new InterceptorManager<RequestOptions>();
  const responseInterceptors = new InterceptorManager<any>();

  url = import.meta.env.VITE_BASE_URL + url;
  // 添加请求拦截器
  requestInterceptors.use({
    onFulfilled: (options): RequestOptions => {
      // 处理请求;
      const access_token = options.headers?.get("Authorization") ?? null;
      if (!access_token) {
        goto("/auth/login");
      }
      console.log("请求拦截，处理请求");
      return options;
    },
    onRejected: (error: any): any => {
      console.log("请求拦截器: 处理错误", error);
      return error;
    },
  });

  // 添加响应拦截器
  responseInterceptors.use({
    onFulfilled: (response): T => {
      // 处理响应
      console.log("响应拦截器:处理响应");
      if (response.status === 301) {
        goto("/");
      }
      return response;
    },
    onRejected: (error) => {
      console.log("响应拦截器: 处理错误", error);
      return error;
    },
  });

  // todo
  requestInterceptors.forEach(async (interceptor) => {
    options = (await interceptor.onFulfilled?.(options)) ?? options;
  });
  let response = await fetch(url, {
    method: options.method ?? "GET",
    headers: options.headers || {
      "Content-Type": "application/json",
    },
    body: options.body,
  });
  const total = response.headers.get("Content-Length");
  let reader = response.body?.getReader();
  const decode = new TextDecoder();
  let data = "";
  while (true) {
    const process = await reader?.read();
    data += decode.decode(process?.value);
    if (process?.done) {
      break;
    }
    const readLen = process?.value.length ?? 0;

    progress.update((n) => n + readLen / Number(total));
  }

  // 处理响应拦截器，一一遍历所有的响应拦截器，并执行onFulfilled方法，将返回值赋值给response
  responseInterceptors.forEach((intercepor) => {
    response = intercepor.onFulfilled?.(response) ?? response;
  });
  setTimeout(() => {
    progress.set(0);
  }, 200);
  return response;
}

export const get = async <T>(url: string, params: BodyInit): Promise<T> => {
  const response: Promise<T> = await request(url, {
    method: "GET",
    body: params,
  });
  return response;
};
export const post = async <T>(
  url: string,
  body: T,
  postType: PostType = PostType.Json
): Promise<any> => {
  const response: Promise<T> = await request(url, {
    method: "POST",
    body: JSON.stringify(body),
  });
  return response;
};
export const put = async <T>(url: string, body: BodyInit): Promise<T> => {
  const response: Promise<any> = await request(url, {
    method: "PUT",
    body,
  });
  return response;
};
export const del = async <T>(url: string, params: BodyInit): Promise<T> => {
  const response: Promise<any> = await request(url, {
    method: "DELETE",
    body: params,
  });
  return response;
};
