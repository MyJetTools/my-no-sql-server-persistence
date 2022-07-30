interface ILocationStatus {
    id: string,
    compress: boolean
}

interface INonInitializedModel {
    tablesTotal: number,
    tablesLoaded: number,
    filesTotal: number,
    filesLoaded: number,
    initializingSeconds: number,
}



interface IStatus {
    notInitialized: INonInitializedModel,
    initialized: IInitializedStatus
    statusBar: IStatusBarModel;

}

interface IInitializedStatus {
    tables: ITableModel[]
}

interface IStatusBarModel {
    persistAmount: number;
    tcpConnections: number;
    tablesAmount: number;
    httpConnections: number;
    location: ILocationStatus,
    masterNode: string,
    syncQueueSize: number
}

interface ITableModel {
    name: number;
    partitionsCount: number;
    dataSize: number;
    recordsAmount: number;
    expirationIndex: number;
    lastUpdateTime: number;
    lastPersistTime: number;
    nextPersistTime: number;
    lastPersistDuration: number[];
    persistAmount: number;
}