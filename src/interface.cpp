#include <msclr\marshal_cppstd.h>
#include <iostream>
#include <string>

#using <OpenHardwareMonitorLib.dll>

using namespace System;
using namespace OpenHardwareMonitor::Hardware;
using namespace std;
using namespace System::Runtime::InteropServices;

#define EXPORT extern "C" __declspec(dllexport)


EXPORT void* Computer_new() {
	Computer^ computer = gcnew Computer();
	computer->CPUEnabled = true;

	IntPtr ptr = static_cast<IntPtr>(GCHandle::Alloc(computer));

	return (void*)ptr;
}

EXPORT void Computer_delete(void* computerPtr) {
	auto handle = GCHandle::FromIntPtr((IntPtr)computerPtr);
	handle.Free();
}


EXPORT char* Computer_GetReport(void* computerPtr) {
	auto handle = GCHandle::FromIntPtr((IntPtr)computerPtr);
	auto computer = (Computer^)(handle.Target);

	auto str = msclr::interop::marshal_as<std::string>(computer->GetReport());
	size_t length = str.length();

	char* c_str = new char[length + 1];
	strcpy(c_str, str.c_str());


	return c_str;
}

EXPORT void char_ptr_delete(char* ptr) {
	delete[] ptr;
}



EXPORT void Computer_Open(void* computerPtr) {
	auto handle = GCHandle::FromIntPtr((IntPtr)computerPtr);
	auto computer = (Computer^)(handle.Target);
	return computer->Open();
}



EXPORT void Computer_UpdateAll(void* computerPtr) {
	auto handle = GCHandle::FromIntPtr((IntPtr)computerPtr);
	auto computer = (Computer^)(handle.Target);

	for each (auto hardwareItem in computer->Hardware) {
		if (hardwareItem->HardwareType == HardwareType::CPU) {
			hardwareItem->Update();
			for each (auto subHardware in hardwareItem->SubHardware) {
				subHardware->Update();
			}
		}
	}
}

EXPORT float* Computer_GetValues(void* computerPtr) {
	auto handle = GCHandle::FromIntPtr((IntPtr)computerPtr);
	auto computer = (Computer^)(handle.Target);

	static float values[10] = { 0 };

	int i = 0;
	for each (auto hardwareItem in computer->Hardware) {
		if (hardwareItem->HardwareType == HardwareType::CPU) {
			for each (auto sensor in hardwareItem->Sensors) {
				if (sensor->SensorType == SensorType::Temperature) {
					if (sensor->Value.HasValue) {
						values[i] = sensor->Value.Value;
						i += 1;
					}
				}
			}
		}
	}

	return values;
}
