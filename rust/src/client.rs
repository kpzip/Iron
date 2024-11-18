use std::cell::RefCell;
use std::sync::{Mutex, RwLock};
use jni::JNIEnv;
use jni::objects::{JClass, JObject};
use jni::sys::jint;

static mut INSTANCE: Option<Mutex<MinecraftClient>> = None;

//static ENVIRONMENT: RwLock<Option<JNIEnv>> = RwLock::new(None);

static GL_ERROR_DIALOGUE: &'static str = "Please make sure you have up-to-date drivers (see aka.ms/mcdriver for instructions).";
pub struct MinecraftClient<'a> {
    field_46551: i64,
    resourcePackDir: JObject<'a>,
    gameProfileFuture: JObject<'a>,
    textureManager: JObject<'a>,
    dataFixer: JObject<'a>,
    windowProvider: JObject<'a>,
    window: JObject<'a>,
    renderTickCounter: JObject<'a>,
    bufferBuilders: JObject<'a>,
    worldRenderer: JObject<'a>,
    entityRenderDispatcher: JObject<'a>,
    itemRenderer: JObject<'a>,
    particleManager: JObject<'a>,
    searchManager: JObject<'a>,
    session: JObject<'a>,
    textRenderer: JObject<'a>,
    advanceValidatingTextRenderer: JObject<'a>,
    gameRenderer: JObject<'a>,
    debugRenderer: JObject<'a>,
    worldGenProgressTracker: JObject<'a>,
    inGameHud: JObject<'a>,
    options: JObject<'a>,
    creativeHotbarStorage: JObject<'a>,
    mouse: JObject<'a>,
    keyboard: JObject<'a>,
    navigationType: JObject<'a>,
    runDirectory: JObject<'a>,
    gameVersion: JObject<'a>,
    versionType: JObject<'a>,
    networkProxy: JObject<'a>,
    levelStorage: JObject<'a>,
    is64Bit: bool,
    isDemo: bool,
    multiplayerEnabled: bool,
    onlineChatEnabled: bool,
    resourceManager: JObject<'a>,
    defaultResourcePack: JObject<'a>,
    serverResourcePackLoader: JObject<'a>,
    resourcePackManager: JObject<'a>,
    languageManager: JObject<'a>,
    blockColors: JObject<'a>,
    itemColors: JObject<'a>,
    framebuffer: JObject<'a>,
    soundManager: JObject<'a>,
    musicTracker: JObject<'a>,
    fontManager: JObject<'a>,
    splashTextLoader: JObject<'a>,
    videoWarningManager: JObject<'a>,
    regionalComplianciesManager: JObject<'a>,
    authenticationService: JObject<'a>,
    sessionService: JObject<'a>,
    userApiService: JObject<'a>,
    userPropertiesFuture: JObject<'a>,
    skinProvider: JObject<'a>,
    bakedModelManager: JObject<'a>,
    blockRenderManager: JObject<'a>,
    paintingManager: JObject<'a>,
    statusEffectSpriteManager: JObject<'a>,
    guiAtlasManager: JObject<'a>,
    toastManager: JObject<'a>,
    tutorialManager: JObject<'a>,
    socialInteractionsManager: JObject<'a>,
    entityModelLoader: JObject<'a>,
    blockEntityRenderDispatcher: JObject<'a>,
    telemetryManager: JObject<'a>,
    profileKeys: JObject<'a>,
    realmsPeriodicCheckers: JObject<'a>,
    quickPlayLogger: JObject<'a>,
    interactionManager: JObject<'a>,
    world: JObject<'a>,
    player: JObject<'a>,
    server: JObject<'a>,
    integratedServerConnection: JObject<'a>,
    integratedServerRunning: bool,
    cameraEntity: JObject<'a>,
    targetedEntity: JObject<'a>,
    crosshairTarget: JObject<'a>,
    itemUseCooldown: i32,
    attackCooldown: i32,
    paused: bool,
    pausedTickDelta: f32,
    lastMetricsSampleTime: i64,
    nextDebugInfoUpdateTime: i64,
    fpsCounter: i32,
    skipGameRender: bool,
    currentScreen: JObject<'a>,
    overlay: JObject<'a>,
    disconnecting: bool,
    thread: JObject<'a>,
    running: bool,
    crashReportSupplier: JObject<'a>,
    fpsDebugString: JObject<'a>,
    renderTime: i64,
    wireFrame: bool,
    debugChunkInfo: bool,
    debugChunkOcclusion: bool,
    chunkCullingEnabled: bool,
    windowFocused: bool,
    renderTaskQueue: JObject<'a>,
    resourceReloadFuture: JObject<'a>,
    socialInteractionsToast: JObject<'a>,
    profiler: JObject<'a>,
    trackingTick: i32,
    tickTimeTracker: JObject<'a>,
    tickProfilerResult: JObject<'a>,
    recorder: JObject<'a>,
    resourceReloadLogger: JObject<'a>,
    metricsSampleDuration: i64,
    gpuUtilizationPercentage: i64,
    currentGlTimerQuery: JObject<'a>,
    realms32BitWarningChecker: JObject<'a>,
    narratorManager: JObject<'a>,
    messageHandler: JObject<'a>,
    abuseReportContext: JObject<'a>,
    commandHistoryManager: JObject<'a>,
    symlinkFinder: JObject<'a>,
    finishedLoading: bool,
    startTime: i64,
    uptimeInTicks: i64,
    openProfilerSection: JObject<'a>,
}

impl <'a> MinecraftClient<'a> {
    pub fn new(args: JObject) -> MinecraftClient {
        todo!()
    }

    fn on_finished_loading(&mut self, loading_context: Option<JObject>) {
        if !self.finishedLoading {
            self.finishedLoading = true;
            self.collect_load_times(loading_context);
        }
    }

    fn collect_load_times<'local>(&mut self, loading_context: Option<JObject>) {
        //let runnable: JObject = self.onInitFinished(loadingContext);
        //let mut env = ENVIRONMENT.write().unwrap().as_ref().unwrap();
        //env.call_static_method();
    }



}