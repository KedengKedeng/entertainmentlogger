export abstract class DataValidator<T> {
    data: T
    constructor(data: T) {
        this.data = data
    }
    abstract validate(): boolean
}