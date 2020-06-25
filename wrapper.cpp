#include "wrapper.h"

#include "DXProgrammableCapture.h"
#include "WinPixEventRuntime\\pix3.h"


static IDXGraphicsAnalysis *g_AnalysisInterface = nullptr;

HRESULT pix_init_analysis()
{
    return DXGIGetDebugInterface1(0, IID_PPV_ARGS(&g_AnalysisInterface));
}

void pix_shutdown_analysis()
{
    g_AnalysisInterface->Release();
}

void pix_begin_capture()
{
    g_AnalysisInterface->BeginCapture();
}

void pix_end_capture()
{
    g_AnalysisInterface->EndCapture();
}

void pix_begin_event_cmd_list(ID3D12CommandList *command_list, UINT64 color, const char *marker)
{
    PIXBeginEvent(command_list, color, "%s", marker);
}

void pix_end_event_cmd_list(ID3D12CommandList *command_list)
{
    PIXEndEvent(command_list);
}

void pix_begin_event_cmd_queue(ID3D12CommandQueue *command_queue, UINT64 color, const char *marker)
{
    PIXBeginEvent(command_queue, color, "%s", marker);
}

void pix_end_event_cmd_queue(ID3D12CommandQueue *command_queue)
{
    PIXEndEvent(command_queue);
}