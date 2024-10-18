/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AddLibro } from '../models/AddLibro';
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
    /**
     * @param requestBody
     * @returns Libro Libro successfully added
     * @throws ApiError
     */
    public static addLibro(
        requestBody: AddLibro,
    ): CancelablePromise<Libro> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/libros',
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                400: `Invalid input`,
            },
        });
    }
}
