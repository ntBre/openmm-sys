#include <OpenMMCWrapper.h>

int main() {
  OpenMM_System* system = OpenMM_System_create();
  OpenMM_Integrator* integrator = (OpenMM_Integrator*)OpenMM_VerletIntegrator_create(0.1);
  OpenMM_Context* context = OpenMM_Context_create(system, integrator);
}
