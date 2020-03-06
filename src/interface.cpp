#include "interface.hpp"

#include <msclr/marshal_cppstd.h>
#include <iostream>
#include <string>

#using <OpenHardwareMonitorLib.dll>

using namespace System;
using namespace OpenHardwareMonitor::Hardware;
using namespace std;
using namespace System::Runtime::InteropServices;


FFICharPtr CharPtr_new(String^ managedStr) {
	auto str = msclr::interop::marshal_as<std::string>(managedStr);
	size_t length = str.length();

	char* c_str = new char[length + 1];
	strcpy_s(c_str, length + 1, str.c_str());

	return { c_str };
}

EXPORT void GCHandle_Free(FFIGCHandle ffiHandle) {
	auto handle = GCHandle::FromIntPtr(static_cast<IntPtr>(ffiHandle.ptr));
	handle.Free();
}

EXPORT FFIGCHandle Computer_new() {
	Computer^ computer = gcnew Computer();
	computer->CPUEnabled = true;

	void* ptr = static_cast<void*>((IntPtr)GCHandle::Alloc(computer));

	return { ptr };
}

EXPORT FFICharPtr Computer_GetReport(const FFIGCHandle& computerHandle) {
	auto handle = GCHandle::FromIntPtr((IntPtr)computerHandle.ptr);
	auto computer = (Computer^)(handle.Target);

	return CharPtr_new(computer->GetReport());
}

EXPORT void CharPtr_delete(FFICharPtr charPtr) {
	delete[] charPtr.ptr;
}



EXPORT void Computer_Open(const FFIGCHandle& computerHandle) {
	auto handle = GCHandle::FromIntPtr((IntPtr)computerHandle.ptr);
	auto computer = (Computer^)(handle.Target);
	return computer->Open();
}



EXPORT FFIHardwares Computer_GetHardwares(const FFIGCHandle& computerHandle) {
	auto handle = GCHandle::FromIntPtr((IntPtr)computerHandle.ptr);
	auto computer = (Computer^)(handle.Target);

	FFIHardwares hardwares = { 0 };

	for each (auto hardwareItem in computer->Hardware) {
		void* ptr = static_cast<void*>(static_cast<IntPtr>(GCHandle::Alloc(hardwareItem)));

		FFIHardware* hardware = &hardwares.items[hardwares.length];
		hardware->ptr = { ptr };
		hardware->name = CharPtr_new(hardwareItem->Name);
		hardware->hardwareType = static_cast<FFIHardwareType>(hardwareItem->HardwareType);

		hardwares.length += 1;
	}

	return hardwares;
}

EXPORT FFIHardwares Hardware_GetSubHardwares(const FFIGCHandle& hardwareHandle) {
	auto handle = GCHandle::FromIntPtr((IntPtr)hardwareHandle.ptr);
	auto hardware = (IHardware^)(handle.Target);

	FFIHardwares hardwares = { 0 };

	for each (auto hardwareItem in hardware->SubHardware) {
		void* ptr = static_cast<void*>(static_cast<IntPtr>(GCHandle::Alloc(hardwareItem)));

		FFIHardware* hardware = &hardwares.items[hardwares.length];
		hardware->ptr = { ptr };
		hardware->name = CharPtr_new(hardwareItem->Name);
		hardware->hardwareType = static_cast<FFIHardwareType>(hardwareItem->HardwareType);

		hardwares.length += 1;
	}

	return hardwares;
}


EXPORT void Hardware_Update(const FFIGCHandle& hardwareHandle) {
	auto handle = GCHandle::FromIntPtr((IntPtr)hardwareHandle.ptr);
	auto hardware = (IHardware^)(handle.Target);

	hardware->Update();
}



EXPORT FFISensors Hardware_GetSensors(const FFIGCHandle& hardwareHandle) {
	auto handle = GCHandle::FromIntPtr((IntPtr)hardwareHandle.ptr);
	auto hardware = (IHardware^)(handle.Target);

	FFISensors sensors = { 0 };

	for each (auto sensorItem in hardware->Sensors) {
		void* ptr = static_cast<void*>(static_cast<IntPtr>(GCHandle::Alloc(sensorItem)));

		FFISensor* sensor = &sensors.items[sensors.length];
		sensor->ptr = { ptr };
		sensor->name = CharPtr_new(sensorItem->Name);
		sensor->sensorType = static_cast<FFISensorType>(sensorItem->SensorType);

		sensors.length += 1;
	}

	return sensors;
}

EXPORT float Sensor_GetValue(const FFIGCHandle& sensorHandle) {
	auto handle = GCHandle::FromIntPtr((IntPtr)sensorHandle.ptr);
	auto sensor = (ISensor^)(handle.Target);

	return sensor->Value.HasValue ? sensor->Value.Value : 0;
}
