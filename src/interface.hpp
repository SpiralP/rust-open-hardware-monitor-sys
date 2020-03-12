#include <stddef.h>

#define EXPORT extern "C" __declspec(dllexport)

struct FFIGCHandle {
	void* ptr;
};

EXPORT void GCHandle_Free(FFIGCHandle handle);

struct FFIWCharPtr {
	wchar_t* ptr;
};

EXPORT void WCharPtr_delete(FFIWCharPtr wCharPtr);

EXPORT FFIGCHandle Computer_new();

EXPORT FFIWCharPtr Computer_GetReport(const FFIGCHandle& computerHandle);

EXPORT void Computer_Open(const FFIGCHandle& computerHandle);

enum FFIHardwareType {
	Mainboard,
	SuperIO,
	CPU,
	RAM,
	GpuNvidia,
	GpuAti,
	TBalancer,
	Heatmaster,
	HDD
};

struct FFIHardware {
	FFIGCHandle ptr;
	FFIWCharPtr name;
	FFIHardwareType hardwareType;
};

struct FFIHardwares {
	size_t length;
	FFIHardware items[32];
};

EXPORT FFIHardwares Computer_GetHardwares(const FFIGCHandle& computerHandle);
EXPORT FFIHardwares Hardware_GetSubHardwares(const FFIGCHandle& hardwareHandle);

EXPORT void Hardware_Update(const FFIGCHandle& hardwareHandle);


enum FFISensorType {
	Voltage,      // V
	Clock,        // MHz
	Temperature,  // °C
	Load,         // %
	Fan,          // RPM
	Flow,         // L/h
	Control,      // %
	Level,        // %
	Factor,       // 1
	Power,        // W
	Data,         // GB = 2^30 Bytes
	SmallData,    // MB = 2^20 Bytes
	Throughput,   // MB/s = 2^20 Bytes/s
};

struct FFISensor {
	FFIGCHandle ptr;
	FFIWCharPtr name;
	FFISensorType sensorType;
};

struct FFISensors {
	size_t length;
	FFISensor items[32];
};

EXPORT FFISensors Hardware_GetSensors(const FFIGCHandle& hardwareHandle);

EXPORT float Sensor_GetValue(const FFIGCHandle& sensorHandle);
