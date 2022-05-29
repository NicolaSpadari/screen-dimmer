declare interface TauriEvent {
    event: string
    windowLabel: string
    payload: {
        message: number
        id: number
    }
}
