/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { Libro } from '../models/Libro';
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class CrateService {
    /**
     * @returns Libro Todos los libros
     * @throws ApiError
     */
    public static getLibros(): CancelablePromise<Array<Libro>> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/libros',
        });
    }
}
